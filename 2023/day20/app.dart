import 'dart:ffi';
import 'dart:io';
import 'dart:convert';
import 'dart:async';

int lcm(int a, int b) => (a * b) ~/ gcd(a, b);

int gcd(int a, int b) {
  while (b != 0) {
    var t = b;
    b = a % t;
    a = t;
  }
  return a;
}

int solutionPart1(Map<String, Module> modules, bool snd) {
  List<Signal> signals = [Signal("Button", Pulse.Low, "broadcaster")];
  var its = 0;
  var originalState = true;
  var high = 0;
  var low = 0;
  var kr = -1;
  var zs = -1;
  var kf = -1;
  var qk = -1;
  for (var module in modules.entries) {
    if (module.value.type != Type.Conjunction) continue;
    for (var toModule in modules.entries) {
      if (toModule.value.destinations.any((element) => element == module.key)) {
        module.value.memory[toModule.key] = Pulse.Low;
      }
    }
  }
  do {
    its++;
    while (signals.isNotEmpty) {
      var signal = signals.removeAt(0);

      if (signal.pulse == Pulse.High) {
        high++;
      } else {
        low++;
      }

      if (signal.pulse == Pulse.High) {
        switch (signal.source) {
          case "kr":
            if (kr == -1) {
              kr = its;
            } else if (its % kr != 0) {
              print("kr $kr not cycle");
            }
            break;
          case "zs":
            if (zs == -1) {
              zs = its;
            } else if (its % zs != 0) {
              print("zs $zs not cycle");
            }
            break;
          case "kf":
            if (kf == -1) {
              kf = its;
            } else if (its % kf != 0) {
              print("kf $kf not cycle");
            }
            break;
          case "qk":
            if (qk == -1) {
              qk = its;
            } else if (its % qk != 0) {
              print("qk $qk not cycle");
            }
            break;
        }
        if (kr != -1 && zs != -1 && kf != -1 && qk != -1) {
          return lcm(lcm(kr, zs), lcm(kf, qk));
        }
      }

      if (!modules.containsKey(signal.destination)) continue;
      var module = modules[signal.destination]!;
      switch (module.type) {
        case Type.Boradcaster:
          for (var dest in module.destinations) {
            signals.add(Signal(signal.destination, signal.pulse, dest));
          }
          break;
        case Type.FlipFlop:
          if (signal.pulse == Pulse.Low) {
            module.state = module.state == Pulse.Low ? Pulse.High : Pulse.Low;
            for (var dest in module.destinations) {
              signals.add(Signal(signal.destination, module.state, dest));
            }
          }
          break;
        case Type.Conjunction:
          module.memory[signal.source] = signal.pulse;
          var pulse =
              module.memory.values.any((element) => element == Pulse.Low)
                  ? Pulse.High
                  : Pulse.Low;
          for (var dest in module.destinations) {
            signals.add(Signal(signal.destination, pulse, dest));
          }
      }
    }

    originalState = true;
    for (var mod in modules.values) {
      if (mod.type == Type.FlipFlop && mod.state == Pulse.High) {
        originalState = false;
        break;
      }
      if (mod.type == Type.Conjunction &&
          mod.memory.values.any((element) => element == Pulse.High)) {
        originalState = false;
      }
    }
    signals.add(Signal("Button", Pulse.Low, "broadcaster"));
  } while (snd || (its < 1000 && !originalState));

  var factor = (1000 ~/ its);
  return high * factor * low * factor;
}

void main() async {
  var input = await parseInput("input.txt");
  var part = Platform.environment["part"] ?? "part1";

  if (part == "part1") {
    print(solutionPart1(input, false));
  } else if (part == "part2") {
    print(solutionPart1(input, true));
  } else {
    print("Unknown part " + part);
  }
}

Future<Map<String, Module>> parseInput(String path) async {
  var res = Map<String, Module>();
  for (var line in await new File(path)
      .openRead()
      .transform(utf8.decoder)
      .transform(const LineSplitter())
      .toList()) {
    var parts = line.split(" -> ");

    var type = Type.Boradcaster;
    var name = "broadcaster";

    if (parts[0] != "broadcaster") {
      type = parts[0].substring(0, 1) == "%" ? Type.FlipFlop : Type.Conjunction;
      name = parts[0].substring(1);
    }

    var dests = parts[1].split(", ");
    res[name] = Module(type, dests);
  }
  return res;
}

enum Pulse { High, Low }
enum Type { Boradcaster, FlipFlop, Conjunction }

class Module {
  Type type;
  List<String> destinations;
  Pulse state = Pulse.Low;
  Map<String, Pulse> memory = Map();
  Module(this.type, this.destinations);

  @override
  String toString() {
    return "$type, $destinations, $state, $memory";
  }
}

class Signal {
  String source;
  Pulse pulse;
  String destination;
  Signal(this.source, this.pulse, this.destination);
  @override
  String toString() {
    return "$source -$pulse-> $destination";
  }
}
