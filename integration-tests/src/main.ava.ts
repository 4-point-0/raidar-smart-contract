import anyTest, { TestFn } from "ava";
import { NEAR, NearAccount, Worker } from "near-workspaces";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  const worker = await Worker.init();

  const root = worker.rootAccount;

  const raidar = await root.createSubAccount("raidar", {
    initialBalance: NEAR.parse("200 N").toJSON(),
  });

  const bob = await root.createSubAccount("bob", {
    initialBalance: NEAR.parse("3 N").toJSON(),
  });

  const sam = await root.createSubAccount("sam", {
    initialBalance: NEAR.parse("7 N").toJSON(),
  });

  await raidar.deploy("./out/main.wasm");

  await raidar.call(raidar, "new_default_meta", {});

  t.context.worker = worker;
  t.context.accounts = { root, raidar, bob, sam };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to tear down the worker:", error);
  });
});

const createSongs = async (
  accounts: Record<string, NearAccount>,
  numberOfCampaigns: number
) => {
  const { raidar } = accounts;

  for (let i = 1; i <= numberOfCampaigns; i++) {
    await raidar.call(
      raidar,
      "mint_nft",
      {
        data: {
          campaign_id: i.toString(),
          token_id: i.toString(),
          name: `Test Song ${i}`,
          description: `Test Song ${i} description`,
        },
      },
      {
        attachedDeposit: "0",
      }
    );
  }
};

// test("Base URL should update", async (t) => {
//   const { raidar, bob } = t.context.accounts;

//   let metadata: {
//     base_uri: string;
//   } = await raidar.view("nft_metadata");

//   t.deepEqual(
//     metadata.base_uri,
//     "http://raidardevapi-env.eba-wdcsmcsf.eu-central-1.elasticbeanstalk.com/api/v1"
//   );

//   await raidar.call(
//     raidar,
//     "update_base_url",
//     { url: "https://test.com" },
//     { attachedDeposit: "0" }
//   );

//   let newMetadata: {
//     base_uri: string;
//   } = await raidar.view("nft_metadata");

//   t.deepEqual(newMetadata.base_uri, "https://test.com");
// });

test("Should throw error if trying to burn NFT and user doesn't have any NFTs", async (t) => {
  const { raidar, bob } = t.context.accounts;

  let bobNFTs: NearNft[] = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 0);

  const error = await t.throwsAsync(
    raidar.call(
      raidar,
      "burn_nft",
      { account_id: bob.accountId, token_id: "1" },
      { attachedDeposit: "0" }
    )
  );

  t.log(error?.message); // uncomment to see the error message

  t.not(error, undefined); // The account doesn't have any tokens

  bobNFTs = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 0);
});

test("Should throw error if trying to burn NFT that is not owned", async (t) => {
  const { raidar, bob } = t.context.accounts;

  await createSongs(t.context.accounts, 1);

  for (let i = 1; i <= 2; i++) {
    await raidar.call(
      raidar,
      "drop_nft",
      { account_id: bob.accountId, token_id: "1" },
      {
        attachedDeposit: "0",
      }
    );
  }

  let bobNFTs: NearNft[] = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 2);

  const error = await t.throwsAsync(
    raidar.call(
      raidar,
      "burn_nft",
      { account_id: bob.accountId, token_id: "2" },
      { attachedDeposit: "0" }
    )
  );

  t.log(error?.message); // uncomment to see the error message

  t.not(error, undefined); // Token should be owned by the account

  bobNFTs = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 2);
});

test("Single collection, mint and burn", async (t) => {
  const { raidar, bob } = t.context.accounts;

  await createSongs(t.context.accounts, 1);

  const raidarStartBalance = await raidar.availableBalance();

  for (let i = 1; i <= 5; i++) {
    await raidar.call(
      raidar,
      "drop_nft",
      { account_id: bob.accountId, token_id: "1" },
      {
        attachedDeposit: "0",
      }
    );
  }

  const raidarEndBalance = await raidar.availableBalance();

  t.log("raidar start balance:", raidarStartBalance.toHuman());
  t.log("raidar end balance:", raidarEndBalance.toHuman());
  t.log(
    "raidar spent balance:",
    raidarStartBalance.sub(raidarEndBalance).toHuman()
  );

  let bobNFTs: NearNft[] = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 5);

  await raidar.call(
    raidar,
    "burn_nft",
    { account_id: bob.accountId, token_id: "1" },
    { attachedDeposit: "0" }
  );

  t.log("After burn");

  const raidarAfterBurnBalance = await raidar.availableBalance();

  t.log("raidar end balance:", raidarAfterBurnBalance.toHuman());

  bobNFTs = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 0);
});

test("Three collections, mint and burn", async (t) => {
  const { raidar, bob } = t.context.accounts;

  await createSongs(t.context.accounts, 3);

  const raidarStartBalance = await raidar.availableBalance();

  await raidar.call(
    raidar,
    "drop_nft",
    { account_id: bob.accountId, token_id: "1" },
    {
      attachedDeposit: "0",
    }
  );

  await raidar.call(
    raidar,
    "drop_nft",
    { account_id: bob.accountId, token_id: "2" },
    {
      attachedDeposit: "0",
    }
  );

  await raidar.call(
    raidar,
    "drop_nft",
    { account_id: bob.accountId, token_id: "3" },
    {
      attachedDeposit: "0",
    }
  );

  await raidar.call(
    raidar,
    "drop_nft",
    { account_id: bob.accountId, token_id: "3" },
    {
      attachedDeposit: "0",
    }
  );

  const raidarEndBalance = await raidar.availableBalance();

  t.log("raidar start balance:", raidarStartBalance.toHuman());
  t.log("raidar end balance:", raidarEndBalance.toHuman());
  t.log(
    "raidar spent balance:",
    raidarStartBalance.sub(raidarEndBalance).toHuman()
  );

  let bobNFTs: NearNft[] = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 4);

  await raidar.call(
    raidar,
    "burn_nft",
    { account_id: bob.accountId, token_id: "1" },
    { attachedDeposit: "0" }
  );

  t.log("After first burn");

  let raidarAfterBurnBalance = await raidar.availableBalance();

  t.log("raidar end balance:", raidarAfterBurnBalance.toHuman());

  bobNFTs = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 3);

  await raidar.call(
    raidar,
    "burn_nft",
    { account_id: bob.accountId, token_id: "3" },
    { attachedDeposit: "0" }
  );

  t.log("After second burn");

  raidarAfterBurnBalance = await raidar.availableBalance();

  t.log("raidar end balance:", raidarAfterBurnBalance.toHuman());

  bobNFTs = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 1);

  await raidar.call(
    raidar,
    "burn_nft",
    { account_id: bob.accountId, token_id: "2" },
    { attachedDeposit: "0" }
  );

  t.log("After third burn");

  raidarAfterBurnBalance = await raidar.availableBalance();

  t.log("raidar end balance:", raidarAfterBurnBalance.toHuman());

  bobNFTs = await raidar.view("nft_tokens_for_owner", {
    account_id: bob.accountId,
  });

  t.deepEqual(bobNFTs.length, 0);
});

// Models

// Interface that represents the NFT data for NEAR chain based on NEAR NFT standard
interface NearNft {
  token_id: string;
  owner_id: string;
  metadata: NearNftMetadata;
  approved_account_ids: Record<string, number>;
}

// Interface that represents the NFT metadata for NEAR chain based on NEAR NFT standard
interface NearNftMetadata {
  title: string;
  description: string;
  media: string;
  media_hash: string | null;
  copies: number | null;
  issues_at: number | null;
  expires_at: number | null;
  starts_at: number | null;
  updated_at: number | null;
  extra: string | null;
  reference: string | null;
  reference_hash: string | null;
}
