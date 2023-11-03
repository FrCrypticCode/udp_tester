mod udp;
use udp::SockReady;

use std::net::UdpSocket;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use eframe::egui;
use egui::*;

#[tokio::main]
async fn main()->Result<(),eframe::Error>{
    
    // Config Window
    let config:Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));
    let clone_config = config.clone();
    let mut address = String::from("127.0.0.1");
    let mut clone_address1 = address.clone();
    let mut port = String::new();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_simple_native("Config Listener", options.clone(), move|ctx, frame|{
        egui::CentralPanel::default().show(ctx, |ui|{
            let addr_label = ui.label("Adresse : ");
            ui.text_edit_singleline(&mut clone_address1)
                .labelled_by(addr_label.id);
            let port_label = ui.label("Port : ");
            ui.text_edit_singleline(&mut port)
                .labelled_by(port_label.id);
            if ui.button("Confirm").clicked(){
                {
                    let mut d = config.lock().unwrap();
                    d.insert(String::from("addr"), clone_address1.clone());
                    d.insert(String::from("port"),port.clone());
                }
                frame.close();
            }
        });
    }).unwrap();

    let mut clone_address2 = String::new(); 
    let mut clone_port = String::new();

    {
        let d = clone_config.lock().unwrap();
        clone_address2 = d.get(&String::from("addr")).unwrap().clone().to_owned();
        clone_port = d.get(&String::from("port")).unwrap().clone().to_owned();
    }

    address = clone_address2.clone();

    // Thread Listener UDP
    let mut data:Arc<Mutex<HashMap<String, String>>>= Arc::new(Mutex::new(HashMap::new()));
    let cloned_data = data.clone();
    tokio::spawn(async move{
        let mut addr = String::from(clone_address2);
        addr = addr+":"+clone_port.as_str();
        //println!("Addr : {}",addr);
        if let Ok(mut list) = SockReady::new(&addr).await{
            loop{
                SockReady::rec_data(&mut list,&mut data).await;
            }
        }
        else if let Err(err) = SockReady::new(&addr).await{
            panic!("Erreur : {}",err);
        }
    });

    address = address+":0";
    //println!("{}",address);
    let socket = UdpSocket::bind(&address).unwrap();

    // Our application state:
    let mut entry = "".to_string();
    let mut a = String::from(address);
    let mut err_send = false;
    let mut err_msg =String::new();

    eframe::run_simple_native("UDP App", options, move |ctx, frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui|{
                let addr_label = ui.label("Adr: ");
                ui.text_edit_singleline(&mut a)
                    .labelled_by(addr_label.id);
            });
            ui.horizontal(|ui| {
                let entry_label = ui.label("Msg: ");
                ui.text_edit_singleline(&mut entry)
                    .labelled_by(entry_label.id);
            });

            ui.horizontal(|ui|{
                if ui.button("Send").clicked() {
                    match socket.connect(&a){
                        Ok(_x)=>{
                            match socket.send(&entry.as_bytes()){
                                Ok(_x)=>{err_send = false},
                                Err(err)=>{
                                    err_send = true;
                                    err_msg = err.to_string();
                                }
                            }
                            entry = "".to_owned();
                        },
                        Err(err)=>{
                            err_send = true;
                            err_msg = err.to_string();
                        }
                    };
                }
                if ui.button("Quit").clicked(){
                    frame.close();
                }
            });
            ui.separator();
            ui.separator();
            ScrollArea::vertical()
                .show(ui,|ui|{
                    ui.horizontal(|ui|{
                        ui.label("Données reçues :")
                    });
                    ui.separator();
                    if let Ok(d) = cloned_data.lock(){
                        for l in d.iter(){
                            let (_k,v) = l;
                            ui.label(v.to_owned());
                        }
                    }
                });
            ui.horizontal(|ui|{
                if err_send{
                    ui.label(&err_msg);
                }
            });
        });
    })
}