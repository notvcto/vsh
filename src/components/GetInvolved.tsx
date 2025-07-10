import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import SponsorButtons from "./SponsorButtons";
import { Github, Heart, Code, Bug } from "lucide-react";

const GetInvolved = () => {
  return (
    <section id="install" className="py-20">
      <div className="container mx-auto px-4">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-heading font-bold mb-6 gradient-text">
            Get Involved
          </h2>
          <p className="text-xl text-muted-foreground max-w-3xl mx-auto">
            Join the VSH community and help shape the future of command-line interaction.
          </p>
        </div>

        <div className="max-w-4xl mx-auto space-y-12">
          {/* Installation */}
          <Card className="bg-terminal-bg border-primary/20">
            <CardHeader>
              <CardTitle className="font-heading text-2xl flex items-center gap-3">
                <Code className="w-6 h-6 text-primary" />
                Install VSH
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="terminal">
                <div className="terminal-prompt">
                  curl -sSL https://get.vsh.sh | bash
                </div>
              </div>
              <p className="text-muted-foreground">
                VSH is currently in development. Join our GitHub for early access and updates.
              </p>
            </CardContent>
          </Card>

          {/* Contribute */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <Card className="bg-card border-border hover:border-primary/50 transition-all duration-300">
              <CardHeader>
                <CardTitle className="font-heading text-xl flex items-center gap-3">
                  <Github className="w-5 h-5 text-primary" />
                  Contribute Code
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <p className="text-muted-foreground">
                  Help build VSH by contributing code, documentation, or tests. Check out our contribution guidelines.
                </p>
                <Button variant="outline" className="border-primary hover:bg-primary hover:text-primary-foreground" asChild>
                  <a 
                    href="https://github.com/notvcto/vsh/blob/main/CONTRIBUTING.md" 
                    target="_blank" 
                    rel="noopener noreferrer"
                  >
                    View Guidelines
                  </a>
                </Button>
              </CardContent>
            </Card>

            <Card className="bg-card border-border hover:border-primary/50 transition-all duration-300">
              <CardHeader>
                <CardTitle className="font-heading text-xl flex items-center gap-3">
                  <Bug className="w-5 h-5 text-primary" />
                  Report Issues
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <p className="text-muted-foreground">
                  Found a bug or have a feature request? Help us improve VSH by reporting issues on GitHub.
                </p>
                <Button variant="outline" className="border-primary hover:bg-primary hover:text-primary-foreground" asChild>
                  <a 
                    href="https://github.com/notvcto/vsh/issues" 
                    target="_blank" 
                    rel="noopener noreferrer"
                  >
                    Report Issue
                  </a>
                </Button>
              </CardContent>
            </Card>
          </div>

          {/* Sponsor */}
          <Card className="bg-gradient-to-br from-primary/10 to-accent/10 border-primary/20">
            <CardHeader>
              <CardTitle className="font-heading text-2xl flex items-center gap-3 justify-center">
                <Heart className="w-6 h-6 text-primary" />
                Support Development
              </CardTitle>
            </CardHeader>
            <CardContent className="text-center space-y-6">
              <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
                VSH is an open-source project. Your support helps us dedicate more time to development and new features.
              </p>
              <SponsorButtons />
            </CardContent>
          </Card>
        </div>
      </div>
    </section>
  );
};

export default GetInvolved;