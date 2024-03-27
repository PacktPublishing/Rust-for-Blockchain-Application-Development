// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract ContractBVersion2 {
    uint256 public testNumber;
    event Subtracted(uint256 value, uint256 result);

    constructor() {
        testNumber = 42;
    }

    function subtract(uint256 num) public {
        require(testNumber >= num, "Subtraction would underflow");
        testNumber -= num;

        emit Subtracted(num, testNumber);
    }
}
