import { useEffect } from "react";
import EntryPage from "../pages/entrypage.tsx";
import DaoInterface from "../pages/daointerface.tsx";
import Wallets from "../pages/components/Wallets";
import { useWallet } from "@solana/wallet-adapter-react";

export default function IndexPage() {
  const { publicKey } = useWallet();

  useEffect(() => {}, [publicKey]);

  return (
    <div>
      {publicKey ? (
        <DaoInterface />
      ) : (
        <>
          <EntryPage />
          <Wallets />
        </>
      )}
    </div>
  );
}
