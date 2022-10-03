use std::io;
use std::sync::mpsc::sync_channel;
use std::thread::sleep;
use std::time::Duration;
use std::{convert::From, ops::Index};

use anyhow::Result;
use helgoboss_midi::{
    Channel, KeyNumber, RawShortMessage, ShortMessage, ShortMessageFactory, StructuredShortMessage,
    U14, U7,
};
use jack::{MidiWriter, PortFlags, RawMidi};

fn main() -> Result<()> {
    // open client
    let (client, _status) =
        jack::Client::new("text2midi", jack::ClientOptions::NO_START_SERVER).unwrap();

    let (sender, receiver) = sync_channel(64);

    // process logic
    let mut output_port = client
        .register_port("tm_output", jack::MidiOut::default())
        .unwrap();
    // let input_port = client // Future?
    //     .register_port("tm_input", jack::MidiIn::default())
    //     .unwrap();

    // dbg!(client.ports(None, None, PortFlags::empty()));

    // client
    //     .connect_ports_by_name(
    //         "Midi-Bridge:Digital Piano:(capture_0) Digital Piano MIDI 1",
    //         "text2midi:tm_input",
    //     )
    //     .unwrap();

    client
        .connect_ports_by_name("text2midi:tm_output", /*"qsynth:midi_00"*/"Midi-Bridge:Digital Piano:(playback_0) Digital Piano MIDI 1")
        .unwrap();

    // 128 MIDI keys, halve that for biology, 64 of them left.
    // Chords of more than nine probably won't sound great, and
    // so we have (2^9)×(64÷9), I think. 3640 combinations.
    //
    // https://en.wiktionary.org/wiki/Wiktionary:Frequency_lists#English
    //     Top 1,000 words cover 85.5% of all words (24,981,922/29,213,800).
    //     Top 10,000 words cover 97.2% of all words (28,398,152/29,213,800).
    //
    // Now we load what we scraped
    //  https://en.wiktionary.org/wiki/Wiktionary:Frequency_lists/PG/2006/04/1-10000
    //  alert([...document.querySelectorAll("tbody")].map(el=>[...el.children].slice(1).map(x=>x.children[1].innerText)).flat(1).join("\n"))
    let words = include_str!("./words.txt")
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    let cback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let mut output = output_port.writer(ps);
        let om = &mut output;
        let word: String = receiver.recv().unwrap();
        let oidx = words.iter().position(|w| w.eq_ignore_ascii_case(&word));
        eprintln!("===\t{word}");

        // A beat. Pulses per quarter note is such a weird unit.
        // https://en.wikipedia.org/wiki/Pulses_per_quarter_note
        send_struct(om, StructuredShortMessage::TimingClock);
        // TODO(^): I think we need to separate out all the stuff below
        // and send the notes over to this callback over a channel so we
        // can regularly send this TimingClock. Currently it gets
        // blocked by the recv for the words.

        if let Some(idx) = oidx {

            // As the word is more unusual, add on more notes to describe it.
            let mut expo = 1;
            loop {
                // Think of this as base-n, where n?=9
                let div = 9u64.pow(expo);
                let digit = ((idx as u64) / div) % 9;
                eprintln!(r#"{digit} (^{expo}) "{word}""#);
                if digit == 0 {
                    break;
                } else {
                    assert!(digit <= 127);
                    note_on(
                        om,
                        (digit + 50) as u8,
                        /*TODO: (expo * (50 / 4))*/ 127 as u8,
                    );
                    expo += 1;
                }
                // assert!(expo < 4); // 9^4 = 6561 < 128
            }
        }

        jack::Control::Continue
    };

    let active_client = client
        .activate_async((), jack::ClosureProcessHandler::new(cback))
        .unwrap();

    // std::thread::spawn(move || {
    //     while let Ok(m) = receiver.recv() {
    //         println!("{:?}", m);
    //     }
    // });

    let mut user_input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        for word in user_input.split_whitespace() {
            // Oh, to be a simple String floating in sand..
            sender.send(word.trim().to_owned())?;
            sleep(Duration::from_millis(100));
        }
        user_input.clear();
    }

    // optional deactivation
    active_client.deactivate().unwrap();
    Ok(())
}

// Blah blah boring util noise

fn send_struct(o: &mut MidiWriter, msg: StructuredShortMessage) {
    let output_bytes = msg.to_bytes();
    o.write(&RawMidi {
        time: 0,
        bytes: &[output_bytes.0, output_bytes.1.into(), output_bytes.2.into()],
    })
    .unwrap();
}
fn note_on(om: &mut MidiWriter, key: u8, velocity: u8) {
    send_struct(
        om,
        StructuredShortMessage::NoteOn {
            channel: Channel::new(0),
            velocity: U7::new(velocity),
            key_number: KeyNumber::new(key),
        },
    );

    send_struct(
        om,
        StructuredShortMessage::NoteOff {
            channel: Channel::new(0),
            velocity: U7::new(velocity), // Sometimes the off notes also have a velocity? Weird.. but makes sense.
            key_number: KeyNumber::new(key),
        },
    );
}
