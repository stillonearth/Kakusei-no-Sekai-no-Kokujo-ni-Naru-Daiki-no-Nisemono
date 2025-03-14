pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract KAKUSEIToken is ERC20 {

    uint256 constant initialSupply = 1000000 * (10**18);

    constructor() ERC20("Kakusei", "KKS") {
        _mint(msg.sender, initialSupply);
    }
}