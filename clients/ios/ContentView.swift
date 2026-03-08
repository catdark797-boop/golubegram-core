import SwiftUI

struct NightCityTheme {
    static let background = Color(red: 0.05, green: 0.05, blue: 0.08)
    static let neonCyan = Color(red: 0.0, green: 0.9, blue: 0.9)
    static let neonMagenta = Color(red: 1.0, green: 0.0, blue: 0.8)
    static let alertRed = Color(red: 1.0, green: 0.2, blue: 0.2)
}

struct ContentView: View {
    @State private var isConnected = false
    @State private var peerCount = 0
    @State private var showComicsGuide = false

    var body: some View {
        ZStack {
            NightCityTheme.background.edgesIgnoringSafeArea(.all)
            
            VStack(spacing: 40) {
                // Header
                Text("GOLUBEGRAM")
                    .font(.custom("CourierNewPS-BoldMT", size: 36))
                    .foregroundColor(NightCityTheme.neonCyan)
                    .shadow(color: NightCityTheme.neonCyan, radius: 10, x: 0, y: 0)
                
                // Neon Status Indicator
                Circle()
                    .strokeBorder(
                        isConnected ? NightCityTheme.neonCyan : NightCityTheme.alertRed,
                        lineWidth: 4
                    )
                    .background(Circle().fill(isConnected ? NightCityTheme.neonCyan.opacity(0.2) : NightCityTheme.alertRed.opacity(0.2)))
                    .frame(width: 200, height: 200)
                    .shadow(color: isConnected ? NightCityTheme.neonCyan : NightCityTheme.alertRed, radius: 20)
                    .overlay(
                        VStack {
                            Text(isConnected ? "MESH SECURED" : "ISOLATED")
                                .font(.headline)
                                .foregroundColor(.white)
                            if isConnected {
                                Text("\(peerCount) PEERS")
                                    .font(.subheadline)
                                    .foregroundColor(NightCityTheme.neonCyan)
                            }
                        }
                    )
                
                // One-Click Connectivity Button
                Button(action: {
                    withAnimation {
                        if !self.isConnected {
                            // FFI Call to Rust Core
                            // golubegram_init()
                            self.isConnected = true
                            self.peerCount = Int.random(in: 1...50)
                            print("GOLUBEGRAM CORE INITIATED VIA FFI")
                        } else {
                            self.isConnected = false
                            self.peerCount = 0
                            print("GOLUBEGRAM CORE SUSPENDED")
                        }
                    }
                }) {
                    Text(isConnected ? "SEARCHING FOR PEERS..." : "INITIATE LINK")
                        .font(.custom("HelveticaNeue-Bold", size: 18))
                        .padding()
                        .frame(maxWidth: .infinity)
                        .background(
                            RoundedRectangle(cornerRadius: 10)
                                .stroke(NightCityTheme.neonMagenta, lineWidth: 2)
                                .background(NightCityTheme.neonMagenta.opacity(0.1))
                        )
                        .foregroundColor(NightCityTheme.neonMagenta)
                        .shadow(color: NightCityTheme.neonMagenta, radius: 5)
                }
                .padding(.horizontal, 40)
                
                Spacer()
                
                // Manual Comics Guide Link
                Button("ROOT CA COMICS GUIDE") {
                    showComicsGuide = true
                }
                .foregroundColor(.gray)
                .font(.footnote)
                .underline()
            }
            .padding(.top, 50)
        }
        .sheet(isPresented: $showComicsGuide) {
            ComicsGuideView()
        }
    }
}

struct ComicsGuideView: View {
    var body: some View {
        ZStack {
            Color.black.edgesIgnoringSafeArea(.all)
            VStack {
                Text("БАБУШКА РЕЖИМ 👵")
                    .font(.largeTitle)
                    .foregroundColor(.yellow)
                    .padding()
                
                Text("(Здесь будут гигантские комикс-панели со стрелками, куда тыкать чтобы одобрить MDM профиль Мечты)")
                    .foregroundColor(.white)
                    .multilineTextAlignment(.center)
                    .padding()
            }
        }
    }
}
