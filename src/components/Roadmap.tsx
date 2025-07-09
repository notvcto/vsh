import { Card, CardContent } from "@/components/ui/card";
import { CheckCircle, Clock, Sparkles } from "lucide-react";

const Roadmap = () => {
  const roadmapItems = [
    {
      version: "v0.1",
      title: "MVP Release",
      status: "completed",
      icon: CheckCircle,
      description: "Core natural language parsing, basic commands, safety features",
      items: [
        "Natural language command parsing",
        "File system operations",
        "Safety confirmations",
        "POSIX compatibility layer"
      ]
    },
    {
      version: "v0.2",
      title: "Learning Features",
      status: "in-progress",
      icon: Clock,
      description: "Smart explanations, command history, and learning assistance",
      items: [
        "Command explanations",
        "Interactive tutorials",
        "Smart suggestions",
        "Usage analytics"
      ]
    },
    {
      version: "v1.0",
      title: "Plugin System & Polish",
      status: "planned",
      icon: Sparkles,
      description: "Extensible architecture, themes, and production readiness",
      items: [
        "Plugin architecture",
        "Custom themes",
        "Advanced scripting",
        "Performance optimizations"
      ]
    }
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case "completed":
        return "text-green-500";
      case "in-progress":
        return "text-yellow-500";
      case "planned":
        return "text-primary";
      default:
        return "text-muted-foreground";
    }
  };

  const getStatusBg = (status: string) => {
    switch (status) {
      case "completed":
        return "bg-green-500/10 border-green-500/20";
      case "in-progress":
        return "bg-yellow-500/10 border-yellow-500/20";
      case "planned":
        return "bg-primary/10 border-primary/20";
      default:
        return "bg-muted/10 border-border";
    }
  };

  return (
    <section id="roadmap" className="py-20 bg-card/30">
      <div className="container mx-auto px-4">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-heading font-bold mb-6 gradient-text">
            Development Roadmap
          </h2>
          <p className="text-xl text-muted-foreground max-w-3xl mx-auto">
            Follow our journey from MVP to a full-featured natural language shell.
          </p>
        </div>

        <div className="max-w-4xl mx-auto">
          <div className="space-y-8">
            {roadmapItems.map((item, index) => {
              const Icon = item.icon;
              return (
                <Card 
                  key={index} 
                  className={`${getStatusBg(item.status)} transition-all duration-300 hover:shadow-lg`}
                >
                  <CardContent className="p-8">
                    <div className="flex items-start gap-6">
                      <div className={`flex-shrink-0 w-12 h-12 rounded-full flex items-center justify-center ${getStatusBg(item.status)}`}>
                        <Icon className={`w-6 h-6 ${getStatusColor(item.status)}`} />
                      </div>
                      
                      <div className="flex-1">
                        <div className="flex items-center gap-4 mb-3">
                          <span className={`text-sm font-mono px-3 py-1 rounded-full ${getStatusBg(item.status)} ${getStatusColor(item.status)}`}>
                            {item.version}
                          </span>
                          <h3 className="text-2xl font-heading font-bold">
                            {item.title}
                          </h3>
                        </div>
                        
                        <p className="text-muted-foreground mb-4 text-lg">
                          {item.description}
                        </p>
                        
                        <ul className="grid grid-cols-1 md:grid-cols-2 gap-2">
                          {item.items.map((feature, featureIndex) => (
                            <li key={featureIndex} className="flex items-center gap-2 text-sm">
                              <div className={`w-1.5 h-1.5 rounded-full ${item.status === 'completed' ? 'bg-green-500' : 'bg-primary'}`}></div>
                              {feature}
                            </li>
                          ))}
                        </ul>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              );
            })}
          </div>
        </div>
      </div>
    </section>
  );
};

export default Roadmap;