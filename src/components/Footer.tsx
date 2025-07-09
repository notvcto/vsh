import { Github, Heart } from "lucide-react";

const Footer = () => {
  return (
    <footer className="py-12 border-t border-border bg-card/30">
      <div className="container mx-auto px-4">
        <div className="max-w-4xl mx-auto">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 items-center">
            {/* Author */}
            <div className="text-center md:text-left">
              <p className="text-muted-foreground">
                Created by{" "}
                <a 
                  href="https://github.com/notvcto" 
                  target="_blank" 
                  rel="noopener noreferrer"
                  className="text-primary hover:text-primary/80 transition-colors font-medium"
                >
                  v
                </a>
              </p>
            </div>

            {/* Logo and tagline */}
            <div className="text-center">
              <div className="font-heading font-bold text-xl gradient-text mb-2">
                VSH
              </div>
              <p className="text-sm text-muted-foreground italic">
                "If you can talk, you can shell."
              </p>
            </div>

            {/* Links */}
            <div className="flex justify-center md:justify-end gap-6">
              <a
                href="https://github.com/notvcto/vsh"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 text-muted-foreground hover:text-primary transition-colors"
              >
                <Github className="w-5 h-5" />
                <span className="hidden sm:inline">GitHub</span>
              </a>
              <a
                href="https://github.com/sponsors/notvcto"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-2 text-muted-foreground hover:text-primary transition-colors"
              >
                <Heart className="w-5 h-5" />
                <span className="hidden sm:inline">Sponsor</span>
              </a>
            </div>
          </div>

          <div className="border-t border-border pt-8 mt-8">
            <div className="flex flex-col sm:flex-row justify-between items-center gap-4 text-sm text-muted-foreground">
              <p>
                Licensed under{" "}
                <a 
                  href="https://github.com/notvcto/vsh/blob/main/LICENSE" 
                  target="_blank" 
                  rel="noopener noreferrer"
                  className="text-primary hover:text-primary/80 transition-colors"
                >
                  GPLv3
                </a>
              </p>
              <p>© 2025 VSH. Open source and community-driven.</p>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;