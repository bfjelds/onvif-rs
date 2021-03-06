// Based on ptz.wsdl.xml

// targetNamespace="http://www.onvif.org/ver20/ptz/wsdl"

// xmlns:tt="http://www.onvif.org/ver10/schema"
// xmlns:tptz="http://www.onvif.org/ver20/ptz/wsdl"
// xmlns:xs="http://www.w3.org/2001/XMLSchema"

// <xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../../ver10/schema/onvif.xsd"/>

use crate::schema::onvif as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
