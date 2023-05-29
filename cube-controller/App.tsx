import { DeviceMotion, DeviceMotionMeasurement } from "expo-sensors";
import { StatusBar } from "expo-status-bar";
import { useEffect, useRef, useState } from "react";
import { StyleSheet, Text, View } from "react-native";
import { Connection } from "./src/socket/connection";
import {
  MessageType,
  Transform,
  TransformType,
  messageToBytes,
} from "./src/socket/message";

function App() {
  const conn = useRef(new Connection("phone"));
  const [angles, setAngles] = useState<[number, number, number]>([0, 0, 0]);

  useEffect(() => {
    conn.current.connect();

    function listener(event: DeviceMotionMeasurement) {
      const roll = event.rotation.alpha;
      const yaw = event.rotation.beta;
      const pitch = event.rotation.gamma;
      setAngles([roll, pitch, yaw]);
    }
    const sub = DeviceMotion.addListener(listener);

    return () => {
      sub.remove();
      conn.current.disconnect();
    };
  }, []);

  useEffect(() => {
    const msg: Transform = {
      type: MessageType.TRANSFORM,
      transform: {
        type: TransformType.ROTATE,
        data: angles,
      },
    };
    conn.current.send(msg);
  }, [angles]);
  return (
    <View style={styles.container}>
      <Text>Roll: {angles[0]}</Text>
      <Text>Pitch: {angles[1]}</Text>
      <Text>Yaw: {angles[2]}</Text>
      <StatusBar style="auto" />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "#fff",
    alignItems: "center",
    justifyContent: "center",
  },
});

export default App;
