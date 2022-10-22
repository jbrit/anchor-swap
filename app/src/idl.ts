export type Swap = {
    "version": "0.1.0",
    "name": "swap",
    "instructions": [
      {
        "name": "makeSwap",
        "accounts": [
          {
            "name": "jupiterProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "authority",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "swapProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "poolAuthority",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "swapState",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "sourceToken",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "destinationToken",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "ra1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "ra2",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "ra3",
            "isMut": true,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "inAmount",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "minimumOutAmount",
            "type": "u64"
          }
        ]
      }
    ]
  };

  export const IDL: Swap = {
    "version": "0.1.0",
    "name": "swap",
    "instructions": [
      {
        "name": "makeSwap",
        "accounts": [
          {
            "name": "jupiterProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "authority",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "swapProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "poolAuthority",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "swapState",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "sourceToken",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "destinationToken",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "ra1",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "ra2",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "ra3",
            "isMut": true,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "inAmount",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "minimumOutAmount",
            "type": "u64"
          }
        ]
      }
    ]
  };
