#include <WiFi.h>
#include <WebServer.h>

const char* ssid = "";
const char* password = "";
WebServer server(80);

const int RELAY_PIN = 26;

void setup() {
  Serial.begin(115200);

  pinMode(RELAY_PIN, OUTPUT);
  digitalWrite(RELAY_PIN, LOW);

  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("Connecting to WiFi...");
  }

  Serial.println("WiFi connected");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());

  server.on("/on", []() {
    digitalWrite(RELAY_PIN, HIGH);
    server.send(200, "text/plain", "Relay turned on");
  });

  server.on("/off", []() {
    digitalWrite(RELAY_PIN, LOW);
    server.send(200, "text/plain", "Relay turned off");
  });

  server.begin();
}

void loop() {
  server.handleClient();
}
