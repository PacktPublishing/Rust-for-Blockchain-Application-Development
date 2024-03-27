// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract ContractB {
    uint256 public testNumber;

    constructor() {
        testNumber = 42;
    }

    function subtract(uint256 num) public {
        require(testNumber >= num, "Subtraction would underflow");
        testNumber -= num;
    }
}
