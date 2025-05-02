use std::net::UdpSocket;
use std::time::{Duration, SystemTime};

use bevy::MinimalPlugins;
use bevy::prelude::*;
use bevy_app::{App, PluginGroup, ScheduleRunnerPlugin, Update};
use bevy_log::info;
use bevy_renet::RenetServerPlugin;
use bevy_renet::netcode::{
    NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig,
};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use librecraft_shared::add;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 20.0,
        ))),
    );
    app.add_plugins(RenetServerPlugin);

    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);

    app.add_plugins(NetcodeServerPlugin);
    let server_addr = "127.0.0.1:1337".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(transport);

    app.add_systems(Update, send_message);
    app.add_systems(Update, receive_message);
    app.add_systems(Update, handle_events);
    app.add_systems(Update, log_tick_rate);

    app.run();
}

fn log_tick_rate(mut state: Local<CounterState>) {
    if state.count % 60 == 0 {
        println!("{}", state.count);
    }
    state.count += 1;
}

fn send_message(mut server: ResMut<RenetServer>) {
    server.broadcast_message(DefaultChannel::ReliableOrdered, add(2, 2).to_string());
}

fn receive_message(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            info!("message received");
        }
    }
}

fn handle_events(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client {client_id} connected");
            },
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client {client_id} disconnected: {reason}");
            },
        }
    }
}

#[derive(Default)]
struct CounterState {
    count: u32,
}
