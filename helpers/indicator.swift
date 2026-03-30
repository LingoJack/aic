import Cocoa

class IndicatorView: NSView {
    override func draw(_ dirtyRect: NSRect) {
        // Outer ring
        let ring = NSBezierPath(ovalIn: bounds.insetBy(dx: 3, dy: 3))
        ring.lineWidth = 2.5
        NSColor.systemOrange.withAlphaComponent(0.85).setStroke()
        ring.stroke()

        // Center dot
        let dotSize: CGFloat = 5
        let dot = NSRect(
            x: bounds.midX - dotSize / 2,
            y: bounds.midY - dotSize / 2,
            width: dotSize,
            height: dotSize
        )
        NSColor.systemOrange.withAlphaComponent(0.9).setFill()
        NSBezierPath(ovalIn: dot).fill()
    }
}

// --- main ---

guard CommandLine.arguments.count >= 3,
      let x = Double(CommandLine.arguments[1]),
      let y = Double(CommandLine.arguments[2]) else {
    fputs("Usage: aic-indicator <x> <y>\n", stderr)
    exit(1)
}

let app = NSApplication.shared
app.setActivationPolicy(.accessory) // no dock icon

let screenHeight = NSScreen.main?.frame.height ?? 1080
let size: CGFloat = 36
let frame = NSRect(
    x: x - size / 2,
    y: screenHeight - y - size / 2,  // flip Y: macOS screen origin is bottom-left
    width: size,
    height: size
)

let window = NSWindow(
    contentRect: frame,
    styleMask: .borderless,
    backing: .buffered,
    defer: false
)
window.isOpaque = false
window.backgroundColor = .clear
window.level = .screenSaver
window.ignoresMouseEvents = true
window.hasShadow = false
window.contentView = IndicatorView(frame: NSRect(origin: .zero, size: frame.size))
window.orderFrontRegardless()

DispatchQueue.main.asyncAfter(deadline: .now() + 0.45) {
    NSAnimationContext.runAnimationGroup({ ctx in
        ctx.duration = 0.35
        window.animator().alphaValue = 0
    }, completionHandler: {
        app.terminate(nil)
    })
}

app.run()
