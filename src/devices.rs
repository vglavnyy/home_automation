//! This module declares devices (actuators and sensors) used in home
//! automaton project and associated types as well.
//! All devices use International System of Units [SI] in in/out args.
//! TODO: conversion functions between SI and humans should be provided.

#![allow(dead_code)]

use std::collections::BTreeMap;

/// Electrical power in SI units
/// todo: implement Display!
#[derive(Debug)]
struct ElectricalPower {
    /// Active power [Watts]
    p_watts: f32,
    /// Reactive power [VAR]
    q_var: f32,
}

/// temperature in SI unit (Kelvin)
/// todo: implement Display!
#[derive(Debug)]
struct Temperature {
    temperature: f32,
}

#[derive(Debug, PartialEq)]
enum TempSensorState {
    Good,
    Failure,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum SocketState {
    Active,
    Inactive,
    Failure,
    Unknown,
}

pub trait DeviceInfoProvider {
    fn descriptor(&self) -> &str;
    fn get_state(&self) -> String;
}

pub struct SmartSocket {
    id: String,
    state: SocketState,
    /// Holds the average power, valid iff is_valid().
    power: ElectricalPower,
}

pub struct TempSensor {
    id: String,
    state: TempSensorState,
    /// Holds the average temperature, valid iff is_valid().
    temperature: Temperature,
}

#[allow(dead_code)]
impl SmartSocket {
    pub(crate) fn new(network_id: String) -> Self {
        SmartSocket {
            id: format!("smart_socket: {}", network_id),
            state: SocketState::Unknown,
            power: ElectricalPower { p_watts: f32::NAN, q_var: f32::NAN },
        }
    }

    fn turn_state(&mut self) -> Result<(), SocketState> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        !(self.state == SocketState::Unknown || self.state == SocketState::Failure)
    }
}

impl DeviceInfoProvider for SmartSocket {
    fn descriptor(&self) -> &str {
        &*self.id
    }

    fn get_state(&self) -> String {
        if self.is_valid() {
            format!("state: {:?}, power: {:?}", self.state, self.power)
        } else {
            format!("state: {:?}, power: ?", self.state)
        }
    }
}

impl TempSensor {
    pub(crate) fn new(network_id: String) -> Self {
        TempSensor {
            id: format!("temp_sensor: {}", network_id),
            state: TempSensorState::Unknown,
            temperature: Temperature { temperature: f32::NAN },
        }
    }

    fn is_valid(&self) -> bool {
        !(self.state == TempSensorState::Unknown || self.state == TempSensorState::Failure)
    }
}

impl DeviceInfoProvider for TempSensor {
    fn descriptor(&self) -> &str {
        &*self.id
    }

    fn get_state(&self) -> String {
        if self.is_valid() {
            format!("state: {:?}, temp: {:?}", self.state, self.temperature)
        } else {
            format!("state: {:?}, temp: ?", self.state)
        }
    }
}

pub enum SmartDevice {
    SmartSocket(SmartSocket),
    TempSensor(TempSensor),
}

impl SmartDevice {
    fn as_device_info(&self) -> &dyn DeviceInfoProvider {
        // Can it be simplified like C++ static_cast because every type is known?
        match &self {
            SmartDevice::SmartSocket(device) => device as &dyn DeviceInfoProvider,
            SmartDevice::TempSensor(device) => device as &dyn DeviceInfoProvider
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct DeviceId {
    location: String,
    name: String,
}

type DeviceTable = BTreeMap<DeviceId, SmartDevice>;

pub struct SmartHouse {
    devices: DeviceTable,
}

impl SmartHouse {
    pub fn new() -> Self {
        SmartHouse {
            devices: DeviceTable::new()
        }
    }

    pub fn add_device(&mut self, location: String, name: String, device: SmartDevice) {
        self.devices.insert(DeviceId { location, name }, device);
    }

    pub fn create_report(&self) -> Vec<String> {
        let mut result = Vec::new();
        for (key, value) in &self.devices {
            let device_info = value.as_device_info();
            result.push(
                format!("{{ location: {}@{}, {}, {} }}",
                        key.name,
                        key.location,
                        device_info.get_state(),
                        device_info.descriptor()))
        }
        result
    }
}