use sdl2::audio::{AudioCallback, AudioSpec, AudioSpecDesired};
use smallvec::{SmallVec, smallvec};
use std::{
    collections::HashMap,
    time::{Duration, Instant, SystemTime},
};


const MAX_NODE_DEPENDENCIES: usize = 8;

type NodeId = u128;
struct SquareWave {
    freq_node: NodeId,
    freq: f32,
    phase: f32,
    volume: f32,
    spec: AudioSpec,
}

trait SynthNode: AudioCallback<Channel = f32> + DependingNode {}

struct RootNode {
    nodes: HashMap<NodeId, Box<dyn SynthNode>>,
    spec: AudioSpec,
}

impl RootNode {
    fn new(spec: AudioSpec) -> RootNode {
        let mut nodes: HashMap<NodeId, Box<dyn SynthNode>> = HashMap::new();
        nodes.insert(1, Box::new(SquareWave { freq: 330.0, freq_node: 2, phase: 0.0, volume: 0.1, spec}));
        RootNode {
            nodes,
            spec,
        }
    }
}

trait DependingNode {
    fn get_node_dependencies(&self) -> SmallVec<[NodeId; MAX_NODE_DEPENDENCIES]>;
}

impl AudioCallback for RootNode {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let first = self.nodes.get(1).unwrap();
            for dep in first.get_node_dependencies() {

             
            first.callback(out);
    }
}

impl SynthNode for SquareWave {}
impl DependingNode for SquareWave {
    fn get_node_dependencies(&self) -> SmallVec<[NodeId; MAX_NODE_DEPENDENCIES]> {
        smallvec![self.freq_node]
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + (self.freq / self.spec.freq as f32)) % 1.0;
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: Some(32768),     // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("{:?}", spec);

        RootNode::new(spec)
    })?;

    device.resume();

    loop {}
}
