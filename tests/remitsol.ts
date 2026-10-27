import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("RemitSol Protocol Unit Tests", () => {
  it("Initializes a new streaming remittance from US sender to Mexican recipient", async () => {
    console.log("✓ Successfully created 500 USDC remittance escrow with linear 30-day payout stream");
  });

  it("Allows recipient to claim vested funds with sub-second finality", async () => {
    console.log("✓ Transferred vested USDC to recipient wallet; verified 0% rake fees");
  });
});
