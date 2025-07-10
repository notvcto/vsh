import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Github, Terminal } from "lucide-react";

const Hero = () => {
  const [typedText, setTypedText] = useState("");
  const [showCursor, setShowCursor] = useState(true);
  const commands = [
    "list files in projects",
    "copy file.txt to backup/",
    "search 'error' in logs/",
    "show disk space",
    "compress folder projects to archive.tar.gz",
  ];
  const [currentCommand, setCurrentCommand] = useState(0);

  useEffect(() => {
    const command = commands[currentCommand];
    let charIndex = 0;
    setTypedText("");

    // Type the command character by character
    const typeInterval = setInterval(() => {
      if (charIndex <= command.length) {
        setTypedText(command.slice(0, charIndex));
        charIndex++;
      } else {
        clearInterval(typeInterval);

        // Wait 2 seconds, then erase
        setTimeout(() => {
          let eraseIndex = command.length;
          const eraseInterval = setInterval(() => {
            if (eraseIndex >= 0) {
              setTypedText(command.slice(0, eraseIndex));
              eraseIndex--;
            } else {
              clearInterval(eraseInterval);
              // Move to next command
              setCurrentCommand((prev) => (prev + 1) % commands.length);
            }
          }, 30);
        }, 2000);
      }
    }, 100);

    return () => clearInterval(typeInterval);
  }, [currentCommand]);

  // Cursor blinking effect
  useEffect(() => {
    const cursorInterval = setInterval(() => {
      setShowCursor((prev) => !prev);
    }, 530);

    return () => clearInterval(cursorInterval);
  }, []);

  const scrollToSection = (sectionId: string) => {
    const element = document.getElementById(sectionId);
    element?.scrollIntoView({ behavior: "smooth" });
  };

  return (
    <section className="relative min-h-screen flex items-center justify-center hero-background">
      <div className="container mx-auto px-4 text-center">
        <div className="fade-in">
          {/* Logo */}
          <div className="flex justify-center mb-8">
            <div className="relative">
              <div className="w-24 h-24 rounded-2xl bg-gradient-to-br from-primary to-accent flex items-center justify-center glow-purple">
                <Terminal className="w-12 h-12 text-primary-foreground" />
              </div>
            </div>
          </div>

          {/* Main heading */}
          <h1 className="text-6xl md:text-8xl font-heading font-bold mb-6">
            <span className="gradient-text">VSH</span>
          </h1>

          {/* Tagline */}
          <p className="text-2xl md:text-3xl text-muted-foreground mb-12 font-heading">
            "If you can talk, you can shell."
          </p>

          {/* Terminal mockup */}
          <div className="max-w-2xl mx-auto mb-12">
            <div className="terminal">
              <div className="flex items-center gap-2 mb-4 pb-2 border-b border-border">
                <div className="w-3 h-3 rounded-full bg-red-500"></div>
                <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
                <div className="w-3 h-3 rounded-full bg-green-500"></div>
                <span className="text-sm text-muted-foreground ml-4">vsh</span>
              </div>
              <div className="space-y-2">
                <div className="flex items-center font-mono text-terminal-text text-left">
                  <span className="text-green-400">user@host</span>
                  <span className="text-muted-foreground">:</span>
                  <span className="text-blue-400">~/projects</span>
                  <span className="text-primary ml-1">$</span>
                  <span className="ml-2">
                    {typedText}
                    <span
                      className={`inline-block w-2 h-5 bg-primary ml-0 align-middle ${
                        showCursor ? "opacity-100" : "opacity-0"
                      } transition-opacity duration-100`}
                    />
                  </span>
                </div>
              </div>
            </div>
          </div>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
            <Button
              size="lg"
              className="glow-purple font-heading text-lg px-8 py-6"
              onClick={() => scrollToSection("install")}
            >
              Install Now
            </Button>
            <Button
              variant="outline"
              size="lg"
              className="font-heading text-lg px-8 py-6 border-primary hover:bg-primary hover:text-primary-foreground"
              asChild
            >
              <a
                href="https://github.com/notvcto/vsh"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2"
              >
                <Github className="w-5 h-5" />
                View on GitHub
              </a>
            </Button>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Hero;
