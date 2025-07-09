import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Copy, Check } from "lucide-react";

const SyntaxCompare = () => {
  const [copiedIndex, setCopiedIndex] = useState<number | null>(null);

  const examples = [
    {
      natural: "list files in projects",
      classic: "ls projects/",
      description: "List directory contents"
    },
    {
      natural: "copy file.txt to backup/",
      classic: "cp file.txt backup/",
      description: "Copy files"
    },
    {
      natural: "search 'error' in logs/",
      classic: "grep 'error' logs/*",
      description: "Search text in files"
    },
    {
      natural: "show disk space",
      classic: "df -h",
      description: "Display disk usage"
    },
    {
      natural: "find large files",
      classic: "find . -size +100M",
      description: "Find files by size"
    },
    {
      natural: "compress folder projects to archive.tar.gz",
      classic: "tar -czf archive.tar.gz projects/",
      description: "Create compressed archive"
    }
  ];

  const copyToClipboard = async (text: string, index: number) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedIndex(index);
      setTimeout(() => setCopiedIndex(null), 2000);
    } catch (err) {
      console.error('Failed to copy text: ', err);
    }
  };

  return (
    <section id="syntax" className="py-20 bg-card/50">
      <div className="container mx-auto px-4">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-heading font-bold mb-6 gradient-text">
            Natural vs Classic
          </h2>
          <p className="text-xl text-muted-foreground max-w-3xl mx-auto">
            Say what you mean. VSH understands natural language and translates it to shell commands.
          </p>
        </div>

        <div className="max-w-6xl mx-auto">
          <Tabs defaultValue="natural" className="w-full">
            <TabsList className="grid w-full grid-cols-2 mb-8 bg-secondary">
              <TabsTrigger value="natural" className="font-heading text-lg">
                Natural Mode
              </TabsTrigger>
              <TabsTrigger value="classic" className="font-heading text-lg">
                Classic Mode
              </TabsTrigger>
            </TabsList>

            <TabsContent value="natural" className="space-y-4">
              {examples.map((example, index) => (
                <Card key={index} className="bg-code-bg border-border hover:border-primary/50 transition-all duration-300">
                  <CardContent className="p-6">
                    <div className="flex items-center justify-between">
                      <div className="flex-1">
                        <div className="flex items-center gap-4 mb-2">
                          <span className="text-primary font-mono">$</span>
                          <code className="text-lg font-mono text-terminal-text">
                            {example.natural}
                          </code>
                        </div>
                        <p className="text-sm text-muted-foreground ml-6">
                          {example.description}
                        </p>
                      </div>
                      <button
                        onClick={() => copyToClipboard(example.natural, index)}
                        className="p-2 hover:bg-secondary rounded-md transition-colors"
                      >
                        {copiedIndex === index ? (
                          <Check className="w-4 h-4 text-green-500" />
                        ) : (
                          <Copy className="w-4 h-4 text-muted-foreground" />
                        )}
                      </button>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </TabsContent>

            <TabsContent value="classic" className="space-y-4">
              {examples.map((example, index) => (
                <Card key={index} className="bg-code-bg border-border hover:border-primary/50 transition-all duration-300">
                  <CardContent className="p-6">
                    <div className="flex items-center justify-between">
                      <div className="flex-1">
                        <div className="flex items-center gap-4 mb-2">
                          <span className="text-primary font-mono">$</span>
                          <code className="text-lg font-mono text-terminal-text">
                            {example.classic}
                          </code>
                        </div>
                        <p className="text-sm text-muted-foreground ml-6">
                          {example.description}
                        </p>
                      </div>
                      <button
                        onClick={() => copyToClipboard(example.classic, index + 100)}
                        className="p-2 hover:bg-secondary rounded-md transition-colors"
                      >
                        {copiedIndex === index + 100 ? (
                          <Check className="w-4 h-4 text-green-500" />
                        ) : (
                          <Copy className="w-4 h-4 text-muted-foreground" />
                        )}
                      </button>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </section>
  );
};

export default SyntaxCompare;