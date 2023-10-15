import { Heading, VStack } from "@chakra-ui/react";
import Wallets from "../pages/components/Wallets";

export default function EntryPage() {
  return (
    <div className="bg-primary">
      <VStack gap={8} mt={16}>
        <div className="flex justify-center items-center h-screen">
          <div className="w-96 bg-black rounded-lg p-8 shadow-md">
            <div className="text-lg font-bold text-center mb-4">DAI - O</div>
            <div className="text-md text-center">
              Welcome to DAI - O. Connect your wallet!
            </div>
            <div className="mb-10"></div>
            <Wallets />
          </div>
        </div>
      </VStack>
    </div>
  );
}
