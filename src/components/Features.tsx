import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { 
  MessageSquare, 
  Shield, 
  Puzzle, 
  BookOpen, 
  Palette, 
  Zap 
} from "lucide-react";

const Features = () => {
  const features = [
    {
      icon: MessageSquare,
      title: "Natural Language Commands",
      description: "Speak or type commands in plain English. VSH understands context and intent, making the shell accessible to everyone."
    },
    {
      icon: Shield,
      title: "Safety-first Execution",
      description: "Built-in safeguards prevent destructive operations. VSH asks for confirmation on dangerous commands and provides undo capabilities."
    },
    {
      icon: Zap,
      title: "POSIX Compatibility",
      description: "Full compatibility with existing shell scripts and commands. Switch between natural and traditional syntax seamlessly."
    },
    {
      icon: Puzzle,
      title: "Plugin System",
      description: "Extensible architecture supports custom commands and integrations. Build your own natural language extensions."
    },
    {
      icon: BookOpen,
      title: "Smart Explanations",
      description: "Learn as you go with intelligent command explanations. VSH shows what each command does and suggests alternatives."
    },
    {
      icon: Palette,
      title: "Themeable Interface",
      description: "Customize your shell experience with themes, colors, and layouts. Make VSH truly yours."
    }
  ];

  return (
    <section id="features" className="py-20">
      <div className="container mx-auto px-4">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-heading font-bold mb-6 gradient-text">
            Powerful Features
          </h2>
          <p className="text-xl text-muted-foreground max-w-3xl mx-auto">
            VSH combines the power of traditional shells with the accessibility of natural language.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-7xl mx-auto">
          {features.map((feature, index) => {
            const Icon = feature.icon;
            return (
              <Card 
                key={index} 
                className="bg-card border-border hover:border-primary/50 transition-all duration-300 group hover:shadow-lg hover:shadow-primary/20"
              >
                <CardHeader>
                  <div className="w-12 h-12 rounded-lg bg-primary/10 flex items-center justify-center mb-4 group-hover:bg-primary/20 transition-colors">
                    <Icon className="w-6 h-6 text-primary" />
                  </div>
                  <CardTitle className="font-heading text-xl mb-2">
                    {feature.title}
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-muted-foreground leading-relaxed">
                    {feature.description}
                  </p>
                </CardContent>
              </Card>
            );
          })}
        </div>
      </div>
    </section>
  );
};

export default Features;