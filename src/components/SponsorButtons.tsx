import { Button } from "@/components/ui/button";
import { Coffee, Heart, DollarSign } from "lucide-react";

const SponsorButtons = () => {
  const sponsors = [
    {
      name: "Buy Me a Coffee",
      icon: Coffee,
      url: "https://buymeacoffee.com/vcto",
      variant: "default" as const,
      className: "bg-[#FFDD00] hover:bg-[#FFDD00]/90 text-black"
    },
    {
      name: "Ko-fi",
      icon: Heart,
      url: "https://ko-fi.com/notvcto",
      variant: "default" as const,
      className: "bg-[#FF5E5B] hover:bg-[#FF5E5B]/90 text-white"
    },
    {
      name: "GitHub Sponsors",
      icon: DollarSign,
      url: "https://github.com/sponsors/notvcto",
      variant: "outline" as const,
      className: "border-primary hover:bg-primary hover:text-primary-foreground"
    }
  ];

  return (
    <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
      {sponsors.map((sponsor) => {
        const Icon = sponsor.icon;
        return (
          <Button
            key={sponsor.name}
            variant={sponsor.variant}
            size="lg"
            className={`font-heading px-6 py-3 ${sponsor.className}`}
            asChild
          >
            <a
              href={sponsor.url}
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2"
            >
              <Icon className="w-5 h-5" />
              {sponsor.name}
            </a>
          </Button>
        );
      })}
    </div>
  );
};

export default SponsorButtons;