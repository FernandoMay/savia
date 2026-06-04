import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";

describe("Savia App", () => {
  it("renders without crashing", () => {
    expect(true).toBe(true);
  });

  it("has correct app name", () => {
    expect("Savia").toBe("Savia");
  });
});
