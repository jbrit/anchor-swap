import "./App.css";
import {
  Connection,
  PublicKey,
  clusterApiUrl,
  ConfirmOptions,
} from "@solana/web3.js";
import { Program, AnchorProvider, BN } from "@project-serum/anchor";
import idl from "./idl.json";

import { PhantomWalletAdapter } from "@solana/wallet-adapter-phantom";
import {
  useWallet,
  AnchorWallet,
  WalletProvider,
  ConnectionProvider,
} from "@solana/wallet-adapter-react";
import {
  WalletModalProvider,
  WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { IDL, Swap } from "./idl";
require("@solana/wallet-adapter-react-ui/styles.css");

const wallets = [
  /* view list of available wallets at https://github.com/solana-labs/wallet-adapter#wallets */
  new PhantomWalletAdapter(),
];
const network = clusterApiUrl(WalletAdapterNetwork.Devnet);

/* create an account  */
const opts = {
  preflightCommitment: "processed",
} as ConfirmOptions;
const programID = new PublicKey(idl.metadata.address);

function App() {
  const {connected, signTransaction, signAllTransactions, publicKey} = useWallet();
  const anchorWallet = {
    signTransaction,
    signAllTransactions,
    publicKey,
  } as AnchorWallet;

  async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const connection = new Connection(
      network,
      opts.preflightCommitment as "processed"
    );
    const provider = new AnchorProvider(connection, anchorWallet, opts);
    return provider;
  }

  async function makeSwap() {
    const provider = await getProvider();
    /* create the program interface combining the idl, program ID, and provider */
    const program = new Program<Swap>(IDL, programID, provider);
    try {
      /* interact with the program via rpc */
      console.log("provider.wallet.publicKey: ", anchorWallet.publicKey?.toBase58());
      const swapTransaction = program.transaction.makeSwap(new BN(0), new BN(0), {
        accounts: {
          jupiterProgram: (new PublicKey(
            "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph"
          )),
          // Mecurial swap program
          swapProgram: new PublicKey(
            "MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky"
          ),
          // Mercurial PAI 3-Pool (USDC-USDT-PAI)
          swapState: new PublicKey(
            "SWABtvDnJwWwAb9CbSA3nv7nTnrtYjrACAVtuP3gyBB"
          ),
          // PAI 3-pool authority
          poolAuthority: new PublicKey(
            "2dc3UgMuVkASzW4sABDjDB5PjFbPTncyECUnZL73bmQR"
          ),
          // USDC token
          sourceToken: new PublicKey(
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
          ),
          // PAI token
          destinationToken: new PublicKey(
            "Ea5SjE2Y6yvCeW5dYTn7PYMuW5ikXkvbGdcmSnXeaLjS"
          ),
          tokenProgram: new PublicKey(
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
          ),
          // my account
          authority: anchorWallet!.publicKey,
        },
      });
     const res = await provider.sendAndConfirm(swapTransaction)
      // const res = await anchorWallet!.signTransaction!(swapTransaction);

      console.log("account: ", res);
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  if (!connected) {
    /* If the user's wallet is not connected, display connect wallet button. */
    return (
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          marginTop: "100px",
        }}
      >
        <WalletMultiButton />
      </div>
    );
  } else {
    return (
      <div className="App">
        {<button onClick={makeSwap}>Swap Or Something</button>}
      </div>
    );
  }
}

const AppWithProvider = () => (
  <ConnectionProvider endpoint={network}>
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <App />
      </WalletModalProvider>
    </WalletProvider>
  </ConnectionProvider>
);

export default AppWithProvider;
