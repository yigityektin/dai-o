import { useState, useEffect } from "react";
import { VStack, Button, Image, Text } from "@chakra-ui/react";
import { useWallet } from "@solana/wallet-adapter-react";

const Wallets = () => {
  const { select, wallets, publicKey, disconnect } = useWallet();
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  return isClient ? (
    <VStack gap={4}>
      {publicKey ? (
        <>{/* <DaoInterface /> */}</>
      ) : wallets.filter((wallet) => wallet.readyState === "Installed").length >
        0 ? (
        wallets
          .filter((wallet) => wallet.readyState === "Installed")
          .map((wallet) => (
            <Button
              key={wallet.adapter.name}
              onClick={() => select(wallet.adapter.name)}
              w="64"
              size="lg"
              fontSize="md"
              leftIcon={
                <Image
                  src={wallet.adapter.icon}
                  alt={wallet.adapter.name}
                  h={6}
                  w={6}
                />
              }
            >
              {wallet.adapter.name}
            </Button>
          ))
      ) : (
        <Text>No wallet found. Please download a supported Solana wallet</Text>
      )}
    </VStack>
  ) : null;
};

export default Wallets;
