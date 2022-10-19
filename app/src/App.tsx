import "./App.css";
import {
  Connection,
  PublicKey,
  clusterApiUrl,
  ConfirmOptions,
  Transaction,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
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
import {
  SolendMarket,
  flashBorrowReserveLiquidityInstruction,
  flashRepayReserveLiquidityInstruction,
  SOLEND_PRODUCTION_PROGRAM_ID,
} from "@solendprotocol/solend-sdk";
require("@solana/wallet-adapter-react-ui/styles.css");

const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);
const wallets = [
  /* view list of available wallets at https://github.com/solana-labs/wallet-adapter#wallets */
  new PhantomWalletAdapter(),
];
const network = clusterApiUrl(WalletAdapterNetwork.Mainnet);

/* create an account  */
const opts = {
  preflightCommitment: "processed",
} as ConfirmOptions;
const programID = new PublicKey(idl.metadata.address);

function App() {
  const { connected, signTransaction, signAllTransactions, publicKey } =
    useWallet();
  const anchorWallet = {
    signTransaction,
    signAllTransactions,
    publicKey,
  } as AnchorWallet;

  const connection = new Connection(
    network,
    opts.preflightCommitment as "processed"
  );
  async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const provider = new AnchorProvider(connection, anchorWallet, opts);
    return provider;
  }

  async function makeSwap() {
    const provider = await getProvider();
    /* create the program interface combining the idl, program ID, and provider */
    const program = new Program<Swap>(IDL, programID, provider);
    try {
      /* interact with the program via rpc */
      console.log(
        "provider.wallet.publicKey: ",
        anchorWallet.publicKey?.toBase58()
      );
      const swapTransaction = program.transaction.makeSwap(
        new BN(1),
        new BN(0),
        {
          accounts: {
            jupiterProgram: new PublicKey(
              "JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph"
            ),
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
            // My USDT token address
            sourceToken: new PublicKey(
              "4PVEWUDq9UVoxcTjbd1vnfCkjTVwcVjR1FaDXMKQav48"
            ),
            // My PAI token address
            destinationToken: new PublicKey(
              "Azt4gz61gxAazEzZJo7WThWojQP9iVAsKVDo46UsL2wc"
            ),
            tokenProgram: new PublicKey(
              "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            ),
            // my account
            authority: anchorWallet!.publicKey,
          },
        }
      );
      const res = await provider.sendAndConfirm(swapTransaction);
      // const res = await anchorWallet!.signTransaction!(swapTransaction);

      console.log("account: ", res);
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  async function initializeTokenAccount() {
    const provider = await getProvider();
    const market = await SolendMarket.initialize(
      connection,
      "production",
      "AAGH44cPMYSq51JZ1rth2AzBqSVass8bxwFxtEQy2L9x"
    );
    const reserve = market.reserves.find(
      (res) => res.config.liquidityToken.symbol === "USDC"
    );
    if (!reserve) {
      throw new Error("Reserve not found");
    }
    const ata = await Token.getAssociatedTokenAddress(
      SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      new PublicKey(reserve!.config.liquidityToken.mint),
      anchorWallet!.publicKey!
    );
    let txa = new Transaction();
    txa.add(
    Token.createAssociatedTokenAccountInstruction(
      SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      new PublicKey(reserve!.config.liquidityToken.mint),
      ata,
      anchorWallet!.publicKey!,
      anchorWallet!.publicKey!,
    ))
    return await provider.sendAndConfirm(txa)
  }

  async function flashLoan() {
    const provider = await getProvider();
    const market = await SolendMarket.initialize(
      connection,
      "production",
      // solend mainnet lending market id
      "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY"
    );
    const reserve = market.reserves.find(
      (res) => res.config.liquidityToken.symbol === "USDT"
    );
    console.log(reserve)
    if (!reserve) {
      throw new Error("Reserve not found");
    }

    const ata = await Token.getAssociatedTokenAddress(
      SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      new PublicKey(reserve!.config.liquidityToken.mint),
      anchorWallet!.publicKey!
    );


    let tx = new Transaction();

    tx.add(
      flashBorrowReserveLiquidityInstruction(
        // liquidity amount - $100
        100000000,
        // source liquidity
        new PublicKey(reserve.config.liquidityAddress),
        // destination liquidity
        ata,
        // reserve
        new PublicKey(reserve.config.address),
        // lending market
        new PublicKey(market.config!.address),
        // lending program
        SOLEND_PRODUCTION_PROGRAM_ID
      ), 
      // Do something to increase your balance here before returning
      flashRepayReserveLiquidityInstruction(
        // liquidity amount (must be the same) - $100
        100000000,
        // borrow instruction index
        0,
        // source liquidity
        ata,
        // destination liquidity
        new PublicKey(reserve.config.liquidityAddress),
        // reserve liquidity fee receiver
        new PublicKey(reserve.config.liquidityFeeReceiverAddress),
        // host fee receiver
        ata,
        // reserve
        new PublicKey(reserve.config.address),
        // lending market
        new PublicKey(market.config!.address),
        // transfer authority
        anchorWallet.publicKey,
        // lending program
        SOLEND_PRODUCTION_PROGRAM_ID
      )
    );

    // hard to make this succeed bc i need tokens to pay off the flash loan fee.
    try {
      await provider.sendAndConfirm(tx);
    } catch (e) {
      console.log("Error found");
      console.log(e);
      return;
    }

    throw new Error("expected a failure!");
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
        {<button onClick={initializeTokenAccount}>Init Account</button>}
        {<button onClick={flashLoan}>Flash Loan</button>}
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
