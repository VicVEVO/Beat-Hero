// use midir::{MidiInput, MidiOutput, Ignore};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Output")?;
    let midi_in = MidiInput::new("My Input")?;
    midi_in.ignore(Ignore::None);

    println!("Ports de sortie MIDI disponibles:");
    for i in 0..midi_out.port_count() {
        println!("{}: {}", i, midi_out.port_name(i)?);
    }

    println!("\nPorts d'entrée MIDI disponibles:");
    for i in 0..midi_in.port_count() {
        println!("{}: {}", i, midi_in.port_name(i)?);
    }

    Ok(())
}
