pragma solidity ^0.8.22;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC721URIStorage} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC721Enumerable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";

contract KakuseiNFT is ERC721, ERC721Enumerable, ERC721URIStorage, Ownable {
    uint256 private _currentIndex;
    string private _baseUri;

    constructor(
        address initialOwner
    ) ERC721("KakuseiNFT", "KKSNFT") Ownable(initialOwner) {
        _currentIndex = 0;
    }

    function _baseURI() internal view override returns (string memory) {
        return _baseUri;
    }

    function _update(address to, uint256 tokenId, address auth)
        internal
        override(ERC721, ERC721Enumerable)
        returns (address)
    {
        return super._update(to, tokenId, auth);
    }

    function _increaseBalance(address account, uint128 value)
        internal
        override(ERC721, ERC721Enumerable)
    {
        super._increaseBalance(account, value);
    }

    function setBaseUri(string calldata newBaseUri) public onlyOwner {
        _baseUri = newBaseUri;
    }

    function safeMint(address to) public onlyOwner returns (uint256) {
        uint256 tokenId = _currentIndex;
        _currentIndex++;

        _safeMint(to, tokenId);

        return tokenId;
    }

    function tokenURI(
        uint256 tokenId
    ) public view override(ERC721, ERC721URIStorage) returns (string memory) {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721, ERC721Enumerable, ERC721URIStorage)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
