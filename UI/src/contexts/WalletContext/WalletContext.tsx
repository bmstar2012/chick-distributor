import { WalletAdapter, WalletError } from "@solana/wallet-adapter-base";
import {
  useWallet,
  WalletProvider as BaseWalletProvider,
} from "@solana/wallet-adapter-react";
import {
  getLedgerWallet,
  getMathWallet,
  getPhantomWallet,
  getSolflareWallet,
  getSolletWallet,
  getSolongWallet,
  getTorusWallet,
} from "@solana/wallet-adapter-wallets";
import { Button } from "antd";
import React, {
  createContext,
  FC,
  ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import { notify } from "oyster-common";
import { DefaultModal } from "../../components";

import "./wallet.less";

export interface WalletModalContextState {
  visible: boolean;
  setVisible: (open: boolean) => void;
}

export const WalletModalContext = createContext<WalletModalContextState>(
  {} as WalletModalContextState
);

export function useWalletModal(): WalletModalContextState {
  return useContext(WalletModalContext);
}

export const WalletModal: FC = () => {
  const { wallets, wallet: selected, select } = useWallet();
  const { visible, setVisible } = useWalletModal();
  const [ , setShowWallets] = useState(false);
  const close = useCallback(() => {
    setVisible(false);
    setShowWallets(false);
  }, [setVisible, setShowWallets]);

  return (
    <div>Test</div>
  );
};

export const WalletModalProvider: FC<{ children: ReactNode }> = ({
  children,
}) => {
  const { publicKey } = useWallet();
  const [connected, setConnected] = useState(!!publicKey);
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    if (publicKey) {
      const base58 = publicKey.toBase58();
      const keyToDisplay =
        base58.length > 20
          ? `${base58.substring(0, 7)}.....${base58.substring(
              base58.length - 7,
              base58.length
            )}`
          : base58;

      notify({
        message: "Wallet update",
        description: `Connected to wallet ${  keyToDisplay}`,
      });
    }
  }, [publicKey]);

  useEffect(() => {
    if (!publicKey && connected) {
      notify({
        message: "Wallet update",
        description: "Disconnected from wallet",
      });
    }
    setConnected(!!publicKey);
  }, [publicKey, connected, setConnected]);

  return (
    <WalletModalContext.Provider
      value={{
        visible,
        setVisible,
      }}
    >
      {children}
      <WalletModal />
    </WalletModalContext.Provider>
  );
};

export const WalletProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const wallets = useMemo(
    () => [
      getPhantomWallet(),
      getSolflareWallet(),
      getTorusWallet({
        options: {
          clientId:
            "BEB_D44HovHuXH0Ace97QVqSu1ahCKndjpGhzhVcMy_9XmDTbHyqTbzQTufcyaN0kFwtlVbfPzJwpJXg94gWJqE",
          uxMode: "redirect",
        },
      }),
      getLedgerWallet(),
      getSolongWallet(),
      getMathWallet(),
      getSolletWallet(),
    ],
    []
  );

  const onError = useCallback((error: WalletError) => {
    console.error(error);
    notify({
      message: "Wallet error",
      description: error.message,
    });
  }, []);

  return (
    <BaseWalletProvider wallets={wallets} onError={onError} autoConnect>
      <WalletModalProvider>{children}</WalletModalProvider>
    </BaseWalletProvider>
  );
};

export type WalletSigner = Pick<
  WalletAdapter,
  "publicKey" | "signTransaction" | "signAllTransactions"
>;
