# zeitdiskrete, dreidimentionale Implementierung eines  PID-Reglers
---

## Wo ist der Algorithmus zu finden?
Im struct [PID](src/pid.rs) in der Member-Funktion `fn anwenden(&mut self, soll: &Vek3, ist: &Vek3) -> Vek3;`