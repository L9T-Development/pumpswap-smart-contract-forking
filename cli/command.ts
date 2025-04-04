import { program } from "commander";
import { PublicKey } from "@solana/web3.js";
import {
  configProject,
  launchToken,
  migrate,
  setClusterConfig,
  swap,
  withdraw,
} from "./scripts";

program.version("0.0.1");

programCommand("config").action(async (directory, cmd) => {
  const { env, keypair, rpc } = cmd.opts();

  console.log("Solana Cluster:", env);
  console.log("Keypair Path:", keypair);
  console.log("RPC URL:", rpc);

  await setClusterConfig(env, keypair, rpc);

  await configProject();
});

programCommand("launch").action(async (directory, cmd) => {
  const { env, keypair, rpc } = cmd.opts();

  console.log("Solana Cluster:", env);
  console.log("Keypair Path:", keypair);
  console.log("RPC URL:", rpc);

  await setClusterConfig(env, keypair, rpc);

  await launchToken();
});

programCommand("swap")
  .option("-t, --token <string>", "token address")
  .option("-a, --amount <number>", "swap amount")
  .option("-s, --style <string>", "0: buy token, 1: sell token")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, token, amount, style } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (token === undefined) {
      console.log("Error token address");
      return;
    }

    if (amount === undefined) {
      console.log("Error swap amount");
      return;
    }

    if (style === undefined) {
      console.log("Error swap style");
      return;
    }

    await swap(new PublicKey(token), amount, style);
  });

programCommand("migrate")
  .option("-t, --token <string>", "token address")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, token } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (token === undefined) {
      console.log("Error token address");
      return;
    }

    await migrate(new PublicKey(token));
  });

programCommand("withdraw")
  .option("-t, --token <string>", "token address")
  .action(async (directory, cmd) => {
    const { env, keypair, rpc, token } = cmd.opts();

    console.log("Solana Cluster:", env);
    console.log("Keypair Path:", keypair);
    console.log("RPC URL:", rpc);

    await setClusterConfig(env, keypair, rpc);

    if (token === undefined) {
      console.log("Error token address");
      return;
    }

    await withdraw(new PublicKey(token));
  });

function programCommand(name: string) {
  return program
    .command(name)
    .option(
      //  mainnet-beta, testnet, devnet
      "-e, --env <string>",
      "Solana cluster env name",
      "devnet"
    )
    .option(
      "-r, --rpc <string>",
      "Solana cluster RPC name",
      "https://devnet.helius-rpc.com/?api-key=7387c4ee-fe6a-43a6-96ea-05e6534aa500"
    )
    .option(
      "-k, --keypair <string>",
      "Solana wallet Keypair Path",
      "./keys/TESTWf3pxDgZ7s8SeVBW19EkgiVbNCBze4KGQHNnHQh.json"
    );
}

program.parse(process.argv);

/*

yarn script config
yarn script launch
yarn script swap -t 84DddAgXUjFLNCCGrumoLrxh5qUgyhm2isx5YTnFrCb6 -a 1000000000 -s 0
yarn script migrate -t 84DddAgXUjFLNCCGrumoLrxh5qUgyhm2isx5YTnFrCb6

*/
