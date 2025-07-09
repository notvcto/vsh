import Hero from "@/components/Hero";
import SyntaxCompare from "@/components/SyntaxCompare";
import Features from "@/components/Features";
import Roadmap from "@/components/Roadmap";
import GetInvolved from "@/components/GetInvolved";
import Footer from "@/components/Footer";

const Index = () => {
  return (
    <div className="min-h-screen bg-background">
      <Hero />
      <SyntaxCompare />
      <Features />
      <Roadmap />
      <GetInvolved />
      <Footer />
    </div>
  );
};

export default Index;
