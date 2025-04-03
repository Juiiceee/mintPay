/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/mint_pay.json`.
 */
export type MintPay = {
  "address": "EXmortUzckHDL9sr874iZQfEqRTnp5DANTDsZGpk7oK5",
  "metadata": {
    "name": "mintPay",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "createTemplate",
      "discriminator": [
        245,
        51,
        247,
        234,
        31,
        9,
        40,
        227
      ],
      "accounts": [
        {
          "name": "template",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  101,
                  109,
                  112,
                  108,
                  97,
                  116,
                  101
                ]
              },
              {
                "kind": "arg",
                "path": "name"
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        },
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initializeCollection",
      "discriminator": [
        112,
        62,
        53,
        139,
        173,
        152,
        98,
        93
      ],
      "accounts": [
        {
          "name": "collectionAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  108,
                  108,
                  101,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "admin",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  100,
                  109,
                  105,
                  110
                ]
              }
            ]
          }
        },
        {
          "name": "collection",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "mplCoreProgram",
          "address": "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        }
      ]
    },
    {
      "name": "initializeMint",
      "discriminator": [
        209,
        42,
        195,
        4,
        129,
        85,
        209,
        44
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "recipient",
          "writable": true
        },
        {
          "name": "mint",
          "writable": true,
          "signer": true
        },
        {
          "name": "template"
        },
        {
          "name": "metaplexCollection",
          "writable": true
        },
        {
          "name": "admin",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  100,
                  109,
                  105,
                  110
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "mplCoreProgram",
          "address": "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "collection",
      "discriminator": [
        48,
        160,
        232,
        205,
        191,
        207,
        26,
        141
      ]
    },
    {
      "name": "template",
      "discriminator": [
        43,
        26,
        88,
        69,
        69,
        96,
        9,
        79
      ]
    }
  ],
  "types": [
    {
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collectionAddress",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "template",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "price",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
