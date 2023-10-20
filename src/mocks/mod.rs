use std::collections::HashMap;

use juniper::InputValue;

use crate::{
    parser::{parse_reports, types::Reports},
    schema::{create_schema, Context},
};

pub fn get_latest_report() -> String {
    let report = r#"
    {
        "nonAddressSpecificData": {
          "associates": {
            "associate": [
              {
                "dob": { "day": 7, "month": 2, "year": 1991 },
                "name": {
                  "forename": "GIHOY",
                  "surname": "HENYJACI",
                  "title": "MRS"
                },
                "sourcedFrom": "ASC"
              }
            ]
          },
          "attributes": [
            { "identifier": "FSC354", "numericValue": 1, "value": "1" },
            { "identifier": "BSC437", "numericValue": 3, "value": "3" },
            { "identifier": "FSC104", "numericValue": 2, "value": "2" },
            {
              "identifier": "LSC325",
              "reason": "No information qualifies for the calculation",
              "value": "C"
            },
            {
              "identifier": "LSC328",
              "reason": "No information qualifies for the calculation",
              "value": "C"
            },
            { "identifier": "LSP510", "numericValue": 0, "value": "0" },
            { "identifier": "LSE510", "reason": "Address not found", "value": "T" },
            { "identifier": "LSN510", "reason": "Address not found", "value": "T" },
            { "identifier": "BSC023", "numericValue": 0, "value": "0" },
            { "identifier": "ESC14", "numericValue": 1, "value": "1" },
            {
              "identifier": "SSC3",
              "reason": "No information in that data type",
              "value": "M"
            },
            {
              "identifier": "SSC12",
              "reason": "No information in that data type",
              "value": "M"
            },
            {
              "identifier": "CSC4",
              "reason": "No information in that data type",
              "value": "M"
            },
            {
              "identifier": "CSP4",
              "reason": "No information in that data type",
              "value": "M"
            },
            { "identifier": "CSE4", "reason": "Address not found", "value": "T" },
            {
              "identifier": "PSC19",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            {
              "identifier": "PSC20",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            {
              "identifier": "PSC21",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            {
              "identifier": "PSC22",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            {
              "identifier": "PSC23",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            {
              "identifier": "PSP30",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            { "identifier": "PSE31", "reason": "", "value": "N" },
            { "identifier": "PSN32", "reason": "", "value": "N" },
            {
              "identifier": "PSC26",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            { "identifier": "PSC27", "reason": "", "value": "A" },
            {
              "identifier": "PSC28",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            {
              "identifier": "PSC29",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            {
              "identifier": "PSC35",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            {
              "identifier": "PSP33",
              "reason": "Characteristic is negative",
              "value": "G"
            },
            { "identifier": "PSE34", "reason": "", "value": "N" }
          ],
          "scores": {
            "score": [
              {
                "positive": true,
                "scoreLabel": "RNOLF04",
                "sourcedFrom": "SCO",
                "value": 538
              },
              {
                "positive": true,
                "scoreLabel": "PSOLF01",
                "sourcedFrom": "SCO",
                "value": 956
              }
            ]
          }
        },
        "provider": "EQ",
        "providerVersion": "batch",
        "soleSearch": {
          "primary": {
            "linkedAddressData": [],
            "suppliedAddressData": [
              {
                "addressMatchStatus": "SINGLE_MATCH",
                "addressSpecificData": {
                  "electoralRollData": {
                    "electoralRoll": [
                      {
                        "annualRegisterPeriod": { "end": "2023", "start": "2021" },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU"
                        },
                        "nameMatchStatus": "A",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2023", "start": "2021" },
                        "name": { "forename": "GIHOY", "surname": "HENYJACI" },
                        "nameMatchStatus": "C",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2022", "start": "2009" },
                        "name": {
                          "forename": "VHOJRYHOA",
                          "middleName": "I",
                          "surname": "IAIOBS"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2022", "start": "2009" },
                        "name": { "forename": "OWOR", "surname": "IAIOBS" },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2009", "start": "2002" },
                        "name": {
                          "forename": "MXIJNAH",
                          "middleName": "Z",
                          "surname": "DETTADM"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "1984", "start": "1983" },
                        "name": {
                          "forename": "RIKI",
                          "surname": "JFCIO",
                          "title": "MS"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "1987", "start": "1983" },
                        "name": {
                          "forename": "PUZAD",
                          "middleName": "J",
                          "surname": "JBYLAWB"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "1987", "start": "1985" },
                        "name": {
                          "forename": "RIKI",
                          "middleName": "H",
                          "surname": "JBYLAWB"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "1992", "start": "1987" },
                        "name": {
                          "forename": "XEKVOAVYRY",
                          "surname": "EZCOXJAR-ZBARR"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "1999", "start": "1987" },
                        "name": {
                          "forename": "XAZXEUX",
                          "surname": "EZCOXJAR-ZBARR"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2002", "start": "1999" },
                        "name": { "forename": "LMXAQFUVBIN", "surname": "LUPH" },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2002", "start": "1999" },
                        "name": {
                          "forename": "NUPY",
                          "middleName": "Y",
                          "surname": "LUPH"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      }
                    ]
                  },
                  "insightData": {
                    "bankDefaultAgreement": [],
                    "basicBankAccount": [],
                    "bridgingFinance": [],
                    "budgetAccount": [],
                    "buyToLetMortgage": [],
                    "chargeCard": [],
                    "commsSupplyAccount": [],
                    "consolidatedDebt": [],
                    "councilArrears": [],
                    "creditCard": [
                      {
                        "accountNumber": "SNr0vi7yepxWTQ6t1TfeFLaMuqxvTzU0YFqyuMyUngo=",
                        "clientNumber": "6WhLLSEdU+CZockroVlI2kT0YmALaeaaL/MDVZczgxQ=",
                        "companyClass": "CC",
                        "companyName": "AMEX GROUP - (I)",
                        "creditLimit": {
                          "limit": { "amount": 15000, "currency": "GBP" },
                          "suppressed": false
                        },
                        "currentBalance": {
                          "balanceAmount": { "amount": 87, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 5, "month": 8, "year": 1985 },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 3, "month": 9, "year": 2023 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 87, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 336, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 294, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 294, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 1158, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": {
                                  "amount": 1158,
                                  "currency": "GBP"
                                },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 1158, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 823, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 823, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 823, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 94, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 94, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 94, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": -88, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": -88, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": -27, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": -27, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": {
                                  "amount": -112,
                                  "currency": "GBP"
                                },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": -112, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 291, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 291, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 291, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 388, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 388, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 388, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 185, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 185, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 185, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "creditLimit": {
                              "limit": { "amount": 15000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "promotionalRate": false,
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          }
                        ],
                        "sourcedFrom": "INY,IBR,ICR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 11, "month": 1, "year": 2022 }
                      }
                    ],
                    "currentAccount": [
                      {
                        "accountNumber": "zGML/Ld93it5j86rAFo2wxM8oGHNdoWJj4WTwoRmkcc=",
                        "clientNumber": "D2Dg+uYQ5i0j7YGwv7jVjYcOUzYfuLxVtg6QodHOe6Q=",
                        "companyClass": "BK",
                        "companyName": "HSBC PLC (I)",
                        "currentBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 8, "month": 12, "year": 1985 },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 31, "month": 8, "year": 2023 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "overdraft": false,
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 19,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 20,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 21,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 22,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 23,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 24,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 25,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 26,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 27,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 28,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 29,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 30,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 31,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 32,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 33,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 34,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 35,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 36,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 37,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 38,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 39,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 40,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 41,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 42,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 43,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 44,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 45,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 46,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 47,
                            "paymentStatus": "ZERO"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 10, "month": 11, "year": 2004 }
                      },
                      {
                        "accountNumber": "3oEmu6B1FCnWguuTc93gXTPtT3NMcaxCKSm2MLOFvMw=",
                        "clientNumber": "k3eYHpx1mc9ywiCIpBrrWh/x7Ho1UB86aJg5y3vnoA8=",
                        "companyClass": "BK",
                        "companyName": "LLOYDS BANK (WAS LLOYDS TSB) (I)",
                        "creditLimit": {
                          "limit": { "amount": 1000, "currency": "GBP" },
                          "suppressed": false
                        },
                        "currentBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 26, "month": 11, "year": 1985 },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 14, "month": 9, "year": 2023 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "overdraft": false,
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 19,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 20,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 21,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 22,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 23,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 24,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 25,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 26,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 27,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 28,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 29,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 30,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 31,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 32,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 33,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 34,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 35,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 36,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 37,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 38,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 39,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 40,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 41,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 42,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 43,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 44,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 45,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 46,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 47,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 28, "month": 6, "year": 2013 }
                      },
                      {
                        "accountNumber": "5YduNv4WxF4SOS0GqS8uh/yOA/TFTgsQT1uH5kAB8RQ=",
                        "clientNumber": "k3eYHpx1mc9ywiCIpBrrWh/x7Ho1UB86aJg5y3vnoA8=",
                        "companyClass": "BK",
                        "companyName": "LLOYDS BANK (WAS LLOYDS TSB) (I)",
                        "currentBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 25, "month": 8, "year": 1985 },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 14, "month": 9, "year": 2023 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "overdraft": false,
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 19,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 20,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 21,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 22,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 23,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 24,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 25,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 26,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 27,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 28,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 29,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 30,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 31,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 32,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 33,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 34,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 35,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 36,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 37,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 38,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 39,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 40,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 41,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 42,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 43,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 44,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 45,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 46,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 47,
                            "paymentStatus": "ZERO"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 6, "month": 6, "year": 2016 }
                      },
                      {
                        "accountNumber": "iMt7bI9kNQtpjsWYsMr69lgUsgyg5XQVMF4dhBknm3E=",
                        "clientNumber": "f+tPFOmpzd57J5Y1HSEsIbINYUCnzF5aBFPINFtCCo4=",
                        "companyClass": "BK",
                        "companyName": "MONZO BANK LIMITED (I)",
                        "currentBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 8, "month": 1, "year": 1985 },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 12, "month": 9, "year": 2023 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "HUSOOT",
                          "surname": "XYHKUHXOXU"
                        },
                        "nameMatchStatus": "A",
                        "overdraft": false,
                        "paymentFrequency": "PERIODICALLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 19,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 20,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 21,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 22,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 23,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 24,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 25,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 26,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 27,
                            "paymentStatus": "ZERO"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 29, "month": 9, "year": 2019 }
                      },
                      {
                        "accountNumber": "LFTlUsWtDduQAo2L7zJOtNKDI86DztlNPL6Fg7iz4+M=",
                        "clientNumber": "f+tPFOmpzd57J5Y1HSEsIbINYUCnzF5aBFPINFtCCo4=",
                        "companyClass": "BK",
                        "companyName": "MONZO BANK LIMITED (I)",
                        "creditLimit": {
                          "limit": { "amount": 1000, "currency": "GBP" },
                          "suppressed": false
                        },
                        "currentBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 20, "month": 1, "year": 1985 },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 12, "month": 9, "year": 2023 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "HUSOOT",
                          "surname": "XYHKUHXOXU"
                        },
                        "nameMatchStatus": "A",
                        "overdraft": false,
                        "paymentFrequency": "PERIODICALLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 2, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 50, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 19,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 20,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 21,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 5, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 22,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 30, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 23,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 24,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 25,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 26,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 26, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 27,
                            "creditLimit": {
                              "limit": { "amount": 1000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 17, "month": 7, "year": 2019 }
                      }
                    ],
                    "fixedTermAgreement": [],
                    "greenDeals": [],
                    "hirePurchase": [],
                    "homeLendingAgreement": [],
                    "insuranceAgreement": [],
                    "localAuthorityHousing": [],
                    "mailOrderAccount": [],
                    "optionAccount": [],
                    "payDayOrShortTermLoan": [],
                    "propertyRental": [],
                    "publicUtilityAccount": [],
                    "rentalAgreement": [],
                    "securedLoan": [
                      {
                        "accountNumber": "kHbepkF0tHD7+oaFLYE/+XMUAuTp58af5EZrYeBtVjs=",
                        "clientNumber": "5MF5etgf/IzguYii51kDtUiiGBX7yiAnTM4assWdEhM=",
                        "companyClass": "BK",
                        "companyName": "Halifax",
                        "currentBalance": {
                          "balanceAmount": { "amount": 521239, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 23, "month": 7, "year": 1985 },
                        "fixedPaymentTerms": {
                          "numberOfPayments": 300,
                          "paymentAmount": { "amount": 2282, "currency": "GBP" }
                        },
                        "flexible": false,
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 12, "month": 9, "year": 2023 },
                        "loanType": "MORTGAGE",
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 521239,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 522652,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 524062,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 525498,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 526903,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 528334,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 529735,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 531219,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 532614,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 534008,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 535428,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 536817,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 538232,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 539616,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 541028,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "paymentStatus": "ZERO"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 30, "month": 6, "year": 2022 }
                      }
                    ],
                    "socialHousingRental": [],
                    "studentLoan": [],
                    "uncategorisedAgreement": [],
                    "unpresentableCheque": [],
                    "unsecuredLoan": [
                      {
                        "accountNumber": "zt6alZd8Gw8CHn/fLohXfBquS9zyU34fw9l2Bn32Jio=",
                        "clientNumber": "ZdUy3iRki2Sh4qovU9loFsic8/Y+YtjWJEdEhAXKsA8=",
                        "companyClass": "RT",
                        "companyName": "MARKS & SPENCER FIN SERV PLC (C) (I)",
                        "currentBalance": {
                          "balanceAmount": { "amount": 1472, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 11, "month": 5, "year": 1985 },
                        "fixedPaymentTerms": {
                          "numberOfPayments": 48,
                          "paymentAmount": { "amount": 210, "currency": "GBP" }
                        },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 3, "month": 9, "year": 2023 },
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 1472, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 1682, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 1892, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 2103, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 2313, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 2523, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 2734, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 2944, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 3154, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 3364, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 3575, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 3785, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 3995, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 4206, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 4416, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 4626, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 4837, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 5047, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 5257, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 5468, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 19,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 5678, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 20,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 5888, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 21,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 6098, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 22,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 6309, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 23,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 6519, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 24,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 6729, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 25,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 6940, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 26,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 7150, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 27,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 7360, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 28,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 7571, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 29,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 7781, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 30,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 7991, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 31,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8202, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 32,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8412, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 33,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8622, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 34,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 8833, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 35,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 9043, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 36,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 9253, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 37,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 9463, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 38,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 9674, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 39,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 9884, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 40,
                            "paymentStatus": "U"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 10094, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 41,
                            "paymentStatus": "U"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 19, "month": 3, "year": 2020 }
                      },
                      {
                        "accountNumber": "tSPyQ+mZayou0q6iQiI3680Jg1GD3aCNuzxB8ph2wX8=",
                        "clientNumber": "7n6dgI3zy+vxzsLawG4LAaINtX07x8h/G8zRHoqkos4=",
                        "companyClass": "BK",
                        "companyName": "MBNA LOANS (I)",
                        "currentBalance": {
                          "balanceAmount": { "amount": 11293, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 9, "month": 12, "year": 1985 },
                        "fixedPaymentTerms": {
                          "numberOfPayments": 60,
                          "paymentAmount": { "amount": 227, "currency": "GBP" }
                        },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 14, "month": 9, "year": 2023 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 11293, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 11470, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 11648, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 11823, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 12000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "paymentStatus": "ZERO"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 1, "month": 4, "year": 2023 }
                      }
                    ],
                    "xmasClub": []
                  },
                  "previousSearches": {
                    "previousCreditSearch": [
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 24, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 15, "month": 3, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 3, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 10, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 6, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 13, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 26, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 13, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 27, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 20, "month": 10, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 13, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 15, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 12, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 15, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 12, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 7, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 7, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 22, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 8, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 1, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 8, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 16, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 14, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 8, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 12, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 13, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 27, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 21, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 27, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 26, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 17, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 26, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 2, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 4, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 16, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 12, "month": 6, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 2, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 20, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 5, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                        "companyName": "BLUE MOTOR FINANCE LTD",
                        "companyType": "FS",
                        "dob": { "day": 12, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 9, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 3, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 6, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 2, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 3, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 4, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 24, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 15, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 5, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 7, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 2, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 7, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 13, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 1, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 3, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "pZApBJ6w6t9j5KWbFbmPfpT+xYddRw1rh+NBX2UrPHo=",
                        "companyName": "HASTINGS LOANS",
                        "companyType": "IN",
                        "dob": { "day": 26, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 22, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                        "companyName": "BLUE MOTOR FINANCE LTD",
                        "companyType": "FS",
                        "dob": { "day": 9, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 12, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 18, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 22, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 11, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 26, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 10, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 24, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 16, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 2, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 3, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 9, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 3, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 8, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 3, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 4, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 24, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 16, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 21, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 21, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 31, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 21, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 31, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 20, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 31, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 14, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 31, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 22, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 31, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 4, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 31, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 21, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 5, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 1, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 21, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 19, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 21, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 4, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 26, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 20, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 25, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 13, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 20, "month": 3, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 2, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 11, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 4, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 4, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 19, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 4, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 19, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 7, "month": 10, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 23, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 7, "month": 10, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "1sHKig3Hx4VDfgLymcLtLdje7l90irsRHH2hP1fjszM=",
                        "companyName": "CASHPLUS",
                        "companyType": "FS",
                        "dob": { "day": 6, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 27, "month": 9, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 24, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 27, "month": 9, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 26, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 27, "month": 9, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 22, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 1, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 5, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 22, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 9, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 25, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                        "companyName": "BLUE MOTOR FINANCE LTD",
                        "companyType": "FS",
                        "dob": { "day": 18, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 21, "month": 3, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 3, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 12, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 20, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 15, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                        "companyName": "BLUE MOTOR FINANCE LTD",
                        "companyType": "FS",
                        "dob": { "day": 9, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 1, "month": 3, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                        "companyName": "BLUE MOTOR FINANCE LTD",
                        "companyType": "FS",
                        "dob": { "day": 26, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      }
                    ],
                    "previousNonCreditSearch": [
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 18, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 21, "month": 7, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 12, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 15, "month": 9, "year": 2023 },
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 26, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 7, "month": 7, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 27, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 2, "month": 9, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 16, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 9, "month": 9, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 16, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 26, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 17, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 14, "month": 7, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 24, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 25, "month": 8, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 21, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 16, "month": 9, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 22, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 19, "month": 11, "year": 2022 },
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 3, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 4, "month": 9, "year": 2023 },
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 2, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 11, "month": 8, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 5, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 4, "month": 8, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 16, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 21, "month": 3, "year": 2023 },
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 24, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 18, "month": 8, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 4, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 30, "month": 6, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 6, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 28, "month": 7, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 18, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 1, "month": 3, "year": 2023 },
                        "sourcedFrom": "ASY"
                      }
                    ]
                  },
                  "rollingRegisterData": {
                    "rollingRegister": [
                      {
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU"
                        },
                        "nameMatchStatus": "A",
                        "recordType": "LOAD",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "RRR",
                        "supplyDate": "092022"
                      },
                      {
                        "name": { "forename": "GIHOY", "surname": "HENYJACI" },
                        "nameMatchStatus": "C",
                        "recordType": "LOAD",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "RRR",
                        "supplyDate": "092022"
                      }
                    ]
                  }
                },
                "index": 1,
                "matchedAddress": {
                  "address": {
                    "addressID": "pzpU8pUJMIOtRgSUqEVWj527/cmIn0nErTB63NuHjkw=",
                    "county": "PIQW GYHZIF",
                    "number": "25447",
                    "postTown": "HORSHAM",
                    "postcode": "SE11 5JA",
                    "street1": "LZOQYQFI GYYW"
                  },
                  "sourcedFrom": "ADO"
                },
                "noticeOfCorrectionOrDisputePresent": false,
                "potentialMatchedAddress": []
              },
              {
                "addressMatchStatus": "SINGLE_MATCH",
                "addressSpecificData": {
                  "electoralRollData": {
                    "electoralRoll": [
                      {
                        "annualRegisterPeriod": { "end": "2023", "start": "2021" },
                        "name": {
                          "forename": "LETONR",
                          "middleName": "SOPUX",
                          "surname": "PUIQQBEPN"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2022", "start": "2020" },
                        "name": { "forename": "GIHOY", "surname": "HENYJACI" },
                        "nameMatchStatus": "C",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2022", "start": "2017" },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "HUSOOT",
                          "surname": "XYHKUHXOXU"
                        },
                        "nameMatchStatus": "A",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2017", "start": "2014" },
                        "name": {
                          "forename": "ZEGWEC",
                          "middleName": "DODL",
                          "surname": "FUZIK"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2017", "start": "2014" },
                        "name": {
                          "forename": "QUUZUO",
                          "middleName": "VUESXU",
                          "surname": "KYDPOG"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "2014", "start": "1989" },
                        "name": { "forename": "BYPSDUUP", "surname": "HUPPUZY" },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "1985", "start": "1983" },
                        "name": {
                          "forename": "UMU",
                          "middleName": "W",
                          "surname": "NUCT",
                          "title": "MS"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "1989", "start": "1986" },
                        "name": {
                          "forename": "ICTATS",
                          "middleName": "N",
                          "surname": "VKWYKKIKS"
                        },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      },
                      {
                        "annualRegisterPeriod": { "end": "1989", "start": "1986" },
                        "name": { "forename": "LOSI", "surname": "VKWYKKIKS" },
                        "nameMatchStatus": "Z",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "ELR"
                      }
                    ]
                  },
                  "insightData": {
                    "bankDefaultAgreement": [],
                    "basicBankAccount": [],
                    "bridgingFinance": [],
                    "budgetAccount": [],
                    "buyToLetMortgage": [],
                    "chargeCard": [],
                    "commsSupplyAccount": [],
                    "consolidatedDebt": [],
                    "councilArrears": [],
                    "creditCard": [
                      {
                        "accountNumber": "cuDI8n9lXiaFB7MaVKIjNWqsNLVPhMTrHEKIe6iez1U=",
                        "clientNumber": "HHVTalQAH9CYjX8vPtUOn5FZPCbqgg+zH0/PjZD93xY=",
                        "companyClass": "FS",
                        "companyName": "LLOYDS BANK (WAS LLOYDS TSB) (I)",
                        "creditLimit": {
                          "limit": { "amount": 10000, "currency": "GBP" },
                          "suppressed": false
                        },
                        "currentBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 18, "month": 9, "year": 1985 },
                        "endDate": { "day": 2, "month": 5, "year": 2018 },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 18, "month": 6, "year": 2018 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "S",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 19,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 20,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 21,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 22,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 23,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 24,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 25,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 26,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "creditLimitChange": "NO_CHANGE",
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 27,
                            "creditLimit": {
                              "limit": { "amount": 10000, "currency": "GBP" },
                              "suppressed": false
                            },
                            "paymentStatus": "ZERO",
                            "statement": {
                              "cashAdvanceCount": 0,
                              "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                              "paymentAmount": { "amount": 0, "currency": "GBP" },
                              "statementBalance": {
                                "balanceAmount": { "amount": 0, "currency": "GBP" },
                                "suppressed": false
                              }
                            }
                          }
                        ],
                        "sourcedFrom": "INY,IBR,ICR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 2, "month": 2, "year": 2016 }
                      }
                    ],
                    "currentAccount": [],
                    "fixedTermAgreement": [],
                    "greenDeals": [],
                    "hirePurchase": [],
                    "homeLendingAgreement": [],
                    "insuranceAgreement": [],
                    "localAuthorityHousing": [],
                    "mailOrderAccount": [],
                    "optionAccount": [],
                    "payDayOrShortTermLoan": [],
                    "propertyRental": [],
                    "publicUtilityAccount": [],
                    "rentalAgreement": [],
                    "securedLoan": [
                      {
                        "accountNumber": "r9jjexGpGIiqxQJx1AODd+N2KFtABRCSglQNZ26UguE=",
                        "clientNumber": "F6TtkHgZMA0k4thIonIcjUfIoOqmln/6oOm4L8BcRfY=",
                        "companyClass": "MS",
                        "companyName": "Lloyds Bank Mortgages Plc",
                        "currentBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 2, "month": 7, "year": 1985 },
                        "endDate": { "day": 10, "month": 6, "year": 2022 },
                        "fixedPaymentTerms": {
                          "numberOfPayments": 0,
                          "paymentAmount": { "amount": 1641, "currency": "GBP" }
                        },
                        "flexible": false,
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 13, "month": 7, "year": 2022 },
                        "loanType": "MORTGAGE",
                        "name": {
                          "forename": "OPIC",
                          "middleName": "K",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "paymentStatus": "S"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 268647,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 269682,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 270735,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 271766,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 272853,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 273879,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 274903,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 7,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 275944,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 8,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 276963,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 9,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 278000,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 10,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 279014,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 11,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 280026,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 12,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 281056,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 13,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 282064,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 14,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 283089,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 15,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 284092,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 16,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 285155,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 17,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 286153,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 18,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 287151,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 19,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 288167,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 20,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 289161,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 21,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 290173,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 22,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 291162,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 23,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 292148,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 24,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 293154,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 25,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 294136,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 26,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 295137,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 27,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 296115,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 28,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 297133,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 29,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 298106,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 30,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 299075,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 31,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 300064,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 32,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 301029,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 33,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 302013,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 34,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 302974,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 35,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 303932,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 36,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 304910,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 37,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 305771,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 38,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 306657,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 39,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 307513,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 40,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 308452,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 41,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 309303,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 42,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 310151,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 43,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 311025,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 44,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 311868,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 45,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 312737,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 46,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": {
                                "amount": 313575,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            },
                            "ageInMonths": 47,
                            "paymentStatus": "ZERO"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 30, "month": 6, "year": 2017 }
                      }
                    ],
                    "socialHousingRental": [],
                    "studentLoan": [],
                    "uncategorisedAgreement": [],
                    "unpresentableCheque": [],
                    "unsecuredLoan": [
                      {
                        "accountNumber": "lsrZ9iCAvJoLUh3DsLmI8ynvqpAUObNt/CHTC7WZGrc=",
                        "clientNumber": "uiNiWE7hUanszyt4z0um7++3Kg4tFTGtCxF19YvjQFs=",
                        "companyClass": "FN",
                        "companyName": "SHAWBROOK BANK (I)",
                        "currentBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "defaultBalance": {
                          "balanceAmount": { "amount": 0, "currency": "GBP" },
                          "suppressed": false
                        },
                        "dob": { "day": 3, "month": 6, "year": 1985 },
                        "endDate": { "day": 9, "month": 10, "year": 2018 },
                        "fixedPaymentTerms": {
                          "numberOfPayments": 6,
                          "paymentAmount": { "amount": 320, "currency": "GBP" }
                        },
                        "insightQuality": {
                          "qualityIndicator1": false,
                          "qualityIndicator2": false
                        },
                        "lastUpdateDate": { "day": 5, "month": 11, "year": 2018 },
                        "name": {
                          "forename": "OPIC",
                          "middleName": "HUSOOT",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "paymentFrequency": "MONTHLY",
                        "paymentHistory": [
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 0,
                            "paymentStatus": "S"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 320, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 1,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 640, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 2,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 960, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 3,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 1280, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 4,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 1600, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 5,
                            "paymentStatus": "ZERO"
                          },
                          {
                            "accountBalance": {
                              "balanceAmount": { "amount": 1920, "currency": "GBP" },
                              "suppressed": false
                            },
                            "ageInMonths": 6,
                            "paymentStatus": "U"
                          }
                        ],
                        "sourcedFrom": "INY,IBR",
                        "startBalance": {
                          "balanceAmount": { "amount": 1920, "currency": "GBP" },
                          "suppressed": false
                        },
                        "startDate": { "day": 9, "month": 4, "year": 2018 }
                      }
                    ],
                    "xmasClub": []
                  },
                  "previousSearches": {
                    "previousCreditSearch": [
                      {
                        "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                        "companyName": "OAKBROOK FINANCE LTD",
                        "companyType": "FN",
                        "dob": { "day": 8, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 7, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "uewaX1KDZA8zwtmECKtjOkSq1E8eO7CfysLRASlIufE=",
                        "companyName": "LIVELEND",
                        "companyType": "FS",
                        "dob": { "day": 27, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "dlRXBa74WZT5EJfsoQaPd8rsktxoAdXE6Llor2roEO8=",
                        "companyName": "AMIGO LOANS LTD T/A REWARDRATE",
                        "companyType": "MN",
                        "dob": { "day": 9, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "uewaX1KDZA8zwtmECKtjOkSq1E8eO7CfysLRASlIufE=",
                        "companyName": "LIVELEND",
                        "companyType": "FS",
                        "dob": { "day": 26, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                        "companyName": "OAKBROOK FINANCE LTD",
                        "companyType": "FN",
                        "dob": { "day": 6, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                        "companyName": "ZOPA CARD WITH CLEARSCORE",
                        "companyType": "MN",
                        "dob": { "day": 21, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                        "companyName": "ZOPA CARD WITH CLEARSCORE",
                        "companyType": "MN",
                        "dob": { "day": 17, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 8, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                        "companyName": "ZOPA CARD WITH CLEARSCORE",
                        "companyType": "MN",
                        "dob": { "day": 13, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 4, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                        "companyName": "OAKBROOK FINANCE LTD",
                        "companyType": "FN",
                        "dob": { "day": 4, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 3, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                        "companyName": "OAKBROOK FINANCE LTD",
                        "companyType": "FN",
                        "dob": { "day": 8, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 1, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                        "companyName": "ZOPA CARD WITH CLEARSCORE",
                        "companyType": "MN",
                        "dob": { "day": 14, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                        "companyName": "OAKBROOK FINANCE LTD",
                        "companyType": "FN",
                        "dob": { "day": 13, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 30, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                        "companyName": "ZOPA CARD WITH CLEARSCORE",
                        "companyType": "MN",
                        "dob": { "day": 23, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 15, "month": 3, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                        "companyName": "ZOPA CARD WITH CLEARSCORE",
                        "companyType": "MN",
                        "dob": { "day": 6, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                        "companyName": "OAKBROOK FINANCE LTD",
                        "companyType": "FN",
                        "dob": { "day": 26, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "uewaX1KDZA8zwtmECKtjOkSq1E8eO7CfysLRASlIufE=",
                        "companyName": "LIVELEND",
                        "companyType": "FS",
                        "dob": { "day": 16, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 7, "month": 10, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                        "companyName": "OAKBROOK FINANCE LTD",
                        "companyType": "FN",
                        "dob": { "day": 6, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 10, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 21, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 5, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 5, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                        "companyName": "BLUE MOTOR FINANCE LTD",
                        "companyType": "FS",
                        "dob": { "day": 9, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 1, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 7, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 18, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 20, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 22, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 8, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 25, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 7, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 8, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 20, "month": 10, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 24, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 27, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 18, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 28, "month": 11, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 21, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 24, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 17, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 11, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 25, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 4, "month": 5, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                        "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 13, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 15, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 15, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 15, "month": 9, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 4, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 14, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 23, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 9, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 23, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 3, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 27, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 8, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 18, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 15, "month": 12, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 2, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 1, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 23, "month": 8, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 20, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 19, "month": 1, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                        "companyName": "BLUE MOTOR FINANCE LTD",
                        "companyType": "FS",
                        "dob": { "day": 26, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 21, "month": 3, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 15, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 1, "month": 4, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 12, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 1, "month": 4, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 6, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 1, "month": 4, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                        "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                        "companyType": "FS",
                        "dob": { "day": 2, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 1, "month": 4, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 17, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 18, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 5, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 4, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 20, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 17, "month": 2, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 27, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 27, "month": 9, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 3, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 27, "month": 9, "year": 2022 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                        "companyName": "CLEARSCORE WITH CAPITAL ONE",
                        "companyType": "CC",
                        "dob": { "day": 4, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 31, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                        "companyName": "BARCLAYCARD",
                        "companyType": "BK",
                        "dob": { "day": 20, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "optIn": true,
                        "searchDate": { "day": 31, "month": 7, "year": 2023 },
                        "searchType": "CREDIT_QUOTATION",
                        "sourcedFrom": "ASY"
                      }
                    ],
                    "previousNonCreditSearch": [
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 27, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 27, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 18, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 23, "month": 2, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 15, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 23, "month": 2, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 25, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 8, "month": 2, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 15, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 8, "month": 2, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "GO5LJLy0r2dTBuNHnUCrGp69LCieQ6g0xeC6q5eTg+4=",
                        "companyName": "HASTINGS INSURANCE SERVICES LTD",
                        "companyType": "IN",
                        "dob": { "day": 2, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 21, "month": 5, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "PvD41LwoaR1LHyepd3DgioyzWQL4hmVlu3EkxnX+4Zk=",
                        "companyName": "EUI LIMITED T/A ADMIRAL",
                        "companyType": "IN",
                        "dob": { "day": 17, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 21, "month": 5, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "v1yCvhOwDzGXEhp+r8KoqiZAkhybVeDV60z1ke6ftD0=",
                        "companyName": "PERCAYSO-INFORM.CO.UK",
                        "companyType": "IN",
                        "dob": { "day": 11, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 21, "month": 5, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 9, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 25, "month": 3, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 2, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 25, "month": 3, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 22, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 21, "month": 3, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 11, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 21, "month": 3, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "weLdQbAYgKf84WCLqmE+OSZP6Rof2a3OV97D9/GhfVI=",
                        "companyName": "HASTINGS INSURANCE SERVICES LTD",
                        "companyType": "IN",
                        "dob": { "day": 20, "month": 5, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 1, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 24, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 17, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 9, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 17, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 23, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 17, "month": 2, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 8, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 17, "month": 2, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 21, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 17, "month": 2, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 11, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 17, "month": 2, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "weLdQbAYgKf84WCLqmE+OSZP6Rof2a3OV97D9/GhfVI=",
                        "companyName": "HASTINGS INSURANCE SERVICES LTD",
                        "companyType": "IN",
                        "dob": { "day": 22, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 5, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 25, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 19, "month": 11, "year": 2022 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 22, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 19, "month": 11, "year": 2022 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 2, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 15, "month": 3, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 5, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 15, "month": 3, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 13, "month": 1, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 19, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 15, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 19, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 18, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 7, "month": 10, "year": 2022 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                        "companyName": "CREDITSPRING",
                        "companyType": "FS",
                        "dob": { "day": 27, "month": 8, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 7, "month": 10, "year": 2022 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 4, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 7, "month": 7, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 2, "month": 3, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 8, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 28, "month": 12, "year": 2022 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 20, "month": 2, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 28, "month": 7, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 17, "month": 11, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 15, "month": 9, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 26, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 30, "month": 6, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 3, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 4, "month": 8, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                        "companyName": "CLEARSCORE TECHNOLOGY LTD",
                        "companyType": "CZ",
                        "dob": { "day": 24, "month": 4, "year": 1985 },
                        "jointApplicant": false,
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 2, "month": 9, "year": 2023 },
                        "searchType": "CREDITFILE_REQUEST",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 22, "month": 9, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 21, "month": 3, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 25, "month": 7, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 1, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 25, "month": 12, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 1, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      },
                      {
                        "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                        "companyName": "ZOPA",
                        "companyType": "FN",
                        "dob": { "day": 19, "month": 6, "year": 1985 },
                        "jointApplicant": false,
                        "name": {
                          "forename": "OPIC",
                          "surname": "XYHKUHXOXU",
                          "title": "MR"
                        },
                        "nameMatchStatus": "A",
                        "searchDate": { "day": 1, "month": 4, "year": 2023 },
                        "searchType": "NO_SEARCH_TYPE",
                        "sourcedFrom": "ASY"
                      }
                    ]
                  },
                  "rollingRegisterData": {
                    "rollingRegister": [
                      {
                        "name": {
                          "forename": "LETONR",
                          "middleName": "SOPUX",
                          "surname": "PUIQQBEPN"
                        },
                        "nameMatchStatus": "Z",
                        "recordType": "LOAD",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "RRR",
                        "supplyDate": "082022"
                      },
                      {
                        "name": { "forename": "GIHOY", "surname": "HENYJACI" },
                        "nameMatchStatus": "C",
                        "recordType": "DELETE",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "RRR",
                        "supplyDate": "092022"
                      },
                      {
                        "name": {
                          "forename": "OPIC",
                          "middleName": "HUSOOT",
                          "surname": "XYHKUHXOXU"
                        },
                        "nameMatchStatus": "A",
                        "recordType": "DELETE",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "RRR",
                        "supplyDate": "092022"
                      },
                      {
                        "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                        "nameMatchStatus": "A",
                        "recordType": "LOAD",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "RRR",
                        "supplyDate": "082017"
                      },
                      {
                        "name": { "forename": "QUUZUO", "surname": "KYDPOG" },
                        "nameMatchStatus": "Z",
                        "recordType": "DELETE",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "RRR",
                        "supplyDate": "072017"
                      },
                      {
                        "name": { "forename": "BYPSDUUP", "surname": "HUPPUZY" },
                        "nameMatchStatus": "Z",
                        "recordType": "DELETE",
                        "seniority": "UNKNOWN",
                        "sourcedFrom": "RRR",
                        "supplyDate": "092014"
                      }
                    ]
                  }
                },
                "index": 2,
                "matchedAddress": {
                  "address": {
                    "addressID": "KjcDX9zzilJxw6RMACDldGzn22Pt5t4onuIknPoHOBg=",
                    "county": "RIJMX",
                    "number": "97363",
                    "postTown": "WATFORD",
                    "postcode": "SE11 5JA",
                    "street1": "BAVMYV SUYT"
                  },
                  "sourcedFrom": "ADO"
                },
                "noticeOfCorrectionOrDisputePresent": false,
                "potentialMatchedAddress": []
              }
            ]
          }
        }
      }
      
    "#;
    report.to_string()
}

pub fn get_historical_report() -> String {
    let report = r#"
  {
      "nonAddressSpecificData": {
        "associates": {
          "associate": [
            {
              "dob": { "day": 7, "month": 2, "year": 1991 },
              "name": {
                "forename": "GIHOY",
                "surname": "HENYJACI",
                "title": "MRS"
              },
              "sourcedFrom": "ASC"
            }
          ]
        },
        "attributes": [
          { "identifier": "FSC354", "numericValue": 1, "value": "1" },
          { "identifier": "BSC437", "numericValue": 3, "value": "3" },
          { "identifier": "FSC104", "numericValue": 2, "value": "2" },
          {
            "identifier": "LSC325",
            "reason": "No information qualifies for the calculation",
            "value": "C"
          },
          {
            "identifier": "LSC328",
            "reason": "No information qualifies for the calculation",
            "value": "C"
          },
          { "identifier": "LSP510", "numericValue": 0, "value": "0" },
          { "identifier": "LSE510", "reason": "Address not found", "value": "T" },
          { "identifier": "LSN510", "reason": "Address not found", "value": "T" },
          { "identifier": "BSC023", "numericValue": 0, "value": "0" },
          { "identifier": "ESC14", "numericValue": 1, "value": "1" },
          {
            "identifier": "SSC3",
            "reason": "No information in that data type",
            "value": "M"
          },
          {
            "identifier": "SSC12",
            "reason": "No information in that data type",
            "value": "M"
          },
          {
            "identifier": "CSC4",
            "reason": "No information in that data type",
            "value": "M"
          },
          {
            "identifier": "CSP4",
            "reason": "No information in that data type",
            "value": "M"
          },
          { "identifier": "CSE4", "reason": "Address not found", "value": "T" },
          {
            "identifier": "PSC19",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          {
            "identifier": "PSC20",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          {
            "identifier": "PSC21",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          {
            "identifier": "PSC22",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          {
            "identifier": "PSC23",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          {
            "identifier": "PSP30",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          { "identifier": "PSE31", "reason": "", "value": "N" },
          { "identifier": "PSN32", "reason": "", "value": "N" },
          {
            "identifier": "PSC26",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          { "identifier": "PSC27", "reason": "", "value": "A" },
          {
            "identifier": "PSC28",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          {
            "identifier": "PSC29",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          {
            "identifier": "PSC35",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          {
            "identifier": "PSP33",
            "reason": "Characteristic is negative",
            "value": "G"
          },
          { "identifier": "PSE34", "reason": "", "value": "N" }
        ],
        "scores": {
          "score": [
            {
              "positive": true,
              "scoreLabel": "RNOLF04",
              "sourcedFrom": "SCO",
              "value": 508
            },
            {
              "positive": true,
              "scoreLabel": "PSOLF01",
              "sourcedFrom": "SCO",
              "value": 936
            }
          ]
        }
      },
      "provider": "EQ",
      "providerVersion": "batch",
      "soleSearch": {
        "primary": {
          "linkedAddressData": [],
          "suppliedAddressData": [
            {
              "addressMatchStatus": "SINGLE_MATCH",
              "addressSpecificData": {
                "electoralRollData": {
                  "electoralRoll": [
                    {
                      "annualRegisterPeriod": { "end": "2023", "start": "2021" },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU"
                      },
                      "nameMatchStatus": "A",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2023", "start": "2021" },
                      "name": { "forename": "GIHOY", "surname": "HENYJACI" },
                      "nameMatchStatus": "C",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2022", "start": "2009" },
                      "name": {
                        "forename": "VHOJRYHOA",
                        "middleName": "I",
                        "surname": "IAIOBS"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2022", "start": "2009" },
                      "name": { "forename": "OWOR", "surname": "IAIOBS" },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2009", "start": "2002" },
                      "name": {
                        "forename": "MXIJNAH",
                        "middleName": "Z",
                        "surname": "DETTADM"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "1984", "start": "1983" },
                      "name": {
                        "forename": "RIKI",
                        "surname": "JFCIO",
                        "title": "MS"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "1987", "start": "1983" },
                      "name": {
                        "forename": "PUZAD",
                        "middleName": "J",
                        "surname": "JBYLAWB"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "1987", "start": "1985" },
                      "name": {
                        "forename": "RIKI",
                        "middleName": "H",
                        "surname": "JBYLAWB"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "1992", "start": "1987" },
                      "name": {
                        "forename": "XEKVOAVYRY",
                        "surname": "EZCOXJAR-ZBARR"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "1999", "start": "1987" },
                      "name": {
                        "forename": "XAZXEUX",
                        "surname": "EZCOXJAR-ZBARR"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2002", "start": "1999" },
                      "name": { "forename": "LMXAQFUVBIN", "surname": "LUPH" },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2002", "start": "1999" },
                      "name": {
                        "forename": "NUPY",
                        "middleName": "Y",
                        "surname": "LUPH"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    }
                  ]
                },
                "insightData": {
                  "bankDefaultAgreement": [],
                  "basicBankAccount": [],
                  "bridgingFinance": [],
                  "budgetAccount": [],
                  "buyToLetMortgage": [],
                  "chargeCard": [],
                  "commsSupplyAccount": [],
                  "consolidatedDebt": [],
                  "councilArrears": [],
                  "creditCard": [
                    {
                      "accountNumber": "SNr0vi7yepxWTQ6t1TfeFLaMuqxvTzU0YFqyuMyUngo=",
                      "clientNumber": "6WhLLSEdU+CZockroVlI2kT0YmALaeaaL/MDVZczgxQ=",
                      "companyClass": "CC",
                      "companyName": "AMEX GROUP - (I)",
                      "creditLimit": {
                        "limit": { "amount": 15000, "currency": "GBP" },
                        "suppressed": false
                      },
                      "currentBalance": {
                        "balanceAmount": { "amount": 87, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 5, "month": 8, "year": 1985 },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 3, "month": 9, "year": 2023 },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 87, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 336, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 294, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 294, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 1158, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": {
                                "amount": 1158,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 1158, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 823, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 823, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 823, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 94, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 94, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 94, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": -88, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": -88, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": -27, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": -27, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": {
                                "amount": -112,
                                "currency": "GBP"
                              },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": -112, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 291, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 291, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 291, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 388, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 388, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 388, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 15,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 16,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 17,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 185, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 185, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 185, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 18,
                          "creditLimit": {
                            "limit": { "amount": 15000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "promotionalRate": false,
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        }
                      ],
                      "sourcedFrom": "INY,IBR,ICR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 11, "month": 1, "year": 2022 }
                    }
                  ],
                  "currentAccount": [
                    {
                      "accountNumber": "3oEmu6B1FCnWguuTc93gXTPtT3NMcaxCKSm2MLOFvMw=",
                      "clientNumber": "k3eYHpx1mc9ywiCIpBrrWh/x7Ho1UB86aJg5y3vnoA8=",
                      "companyClass": "BK",
                      "companyName": "LLOYDS BANK (WAS LLOYDS TSB) (I)",
                      "creditLimit": {
                        "limit": { "amount": 1000, "currency": "GBP" },
                        "suppressed": false
                      },
                      "currentBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 26, "month": 11, "year": 1985 },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 14, "month": 9, "year": 2023 },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "overdraft": false,
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 15,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 16,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 17,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 18,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 19,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 20,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 21,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 22,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 23,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 24,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 25,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 26,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 27,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 28,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 29,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 30,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 31,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 32,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 33,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 34,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 35,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 36,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 37,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 38,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 39,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 40,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 41,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 42,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 43,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 44,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 45,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 46,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 47,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 28, "month": 6, "year": 2013 }
                    },
                    {
                      "accountNumber": "5YduNv4WxF4SOS0GqS8uh/yOA/TFTgsQT1uH5kAB8RQ=",
                      "clientNumber": "k3eYHpx1mc9ywiCIpBrrWh/x7Ho1UB86aJg5y3vnoA8=",
                      "companyClass": "BK",
                      "companyName": "LLOYDS BANK (WAS LLOYDS TSB) (I)",
                      "currentBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 25, "month": 8, "year": 1985 },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 14, "month": 9, "year": 2023 },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "overdraft": false,
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 15,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 16,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 17,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 18,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 19,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 20,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 21,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 22,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 23,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 24,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 25,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 26,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 27,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 28,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 29,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 30,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 31,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 32,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 33,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 34,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 35,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 36,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 37,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 38,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 39,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 40,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 41,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 42,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 43,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 44,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 45,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 46,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 47,
                          "paymentStatus": "ZERO"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 6, "month": 6, "year": 2016 }
                    },
                    {
                      "accountNumber": "iMt7bI9kNQtpjsWYsMr69lgUsgyg5XQVMF4dhBknm3E=",
                      "clientNumber": "f+tPFOmpzd57J5Y1HSEsIbINYUCnzF5aBFPINFtCCo4=",
                      "companyClass": "BK",
                      "companyName": "MONZO BANK LIMITED (I)",
                      "currentBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 8, "month": 1, "year": 1985 },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 12, "month": 9, "year": 2023 },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "HUSOOT",
                        "surname": "XYHKUHXOXU"
                      },
                      "nameMatchStatus": "A",
                      "overdraft": false,
                      "paymentFrequency": "PERIODICALLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 15,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 16,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 17,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 18,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 19,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 20,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 21,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 22,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 23,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 24,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 25,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 26,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 27,
                          "paymentStatus": "ZERO"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 29, "month": 9, "year": 2019 }
                    },
                    {
                      "accountNumber": "LFTlUsWtDduQAo2L7zJOtNKDI86DztlNPL6Fg7iz4+M=",
                      "clientNumber": "f+tPFOmpzd57J5Y1HSEsIbINYUCnzF5aBFPINFtCCo4=",
                      "companyClass": "BK",
                      "companyName": "MONZO BANK LIMITED (I)",
                      "creditLimit": {
                        "limit": { "amount": 1000, "currency": "GBP" },
                        "suppressed": false
                      },
                      "currentBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 20, "month": 1, "year": 1985 },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 12, "month": 9, "year": 2023 },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "HUSOOT",
                        "surname": "XYHKUHXOXU"
                      },
                      "nameMatchStatus": "A",
                      "overdraft": false,
                      "paymentFrequency": "PERIODICALLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 2, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 50, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 15,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 16,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 17,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 18,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 19,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 20,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 21,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 5, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 22,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 30, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 23,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 8, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 24,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 25,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 26,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 26, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 27,
                          "creditLimit": {
                            "limit": { "amount": 1000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 17, "month": 7, "year": 2019 }
                    }
                  ],
                  "fixedTermAgreement": [],
                  "greenDeals": [],
                  "hirePurchase": [],
                  "homeLendingAgreement": [],
                  "insuranceAgreement": [],
                  "localAuthorityHousing": [],
                  "mailOrderAccount": [],
                  "optionAccount": [],
                  "payDayOrShortTermLoan": [],
                  "propertyRental": [],
                  "publicUtilityAccount": [],
                  "rentalAgreement": [],
                  "securedLoan": [
                    {
                      "accountNumber": "kHbepkF0tHD7+oaFLYE/+XMUAuTp58af5EZrYeBtVjs=",
                      "clientNumber": "5MF5etgf/IzguYii51kDtUiiGBX7yiAnTM4assWdEhM=",
                      "companyClass": "BK",
                      "companyName": "Halifax",
                      "currentBalance": {
                        "balanceAmount": { "amount": 521239, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 23, "month": 7, "year": 1985 },
                      "fixedPaymentTerms": {
                        "numberOfPayments": 300,
                        "paymentAmount": { "amount": 2282, "currency": "GBP" }
                      },
                      "flexible": false,
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 12, "month": 9, "year": 2023 },
                      "loanType": "MORTGAGE",
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 521239,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 522652,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 524062,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 525498,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 526903,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 528334,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 529735,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 531219,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 532614,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 534008,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 535428,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 536817,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 538232,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 539616,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 541028,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "paymentStatus": "ZERO"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 30, "month": 6, "year": 2022 }
                    }
                  ],
                  "socialHousingRental": [],
                  "studentLoan": [],
                  "uncategorisedAgreement": [],
                  "unpresentableCheque": [],
                  "unsecuredLoan": [
                    {
                      "accountNumber": "zt6alZd8Gw8CHn/fLohXfBquS9zyU34fw9l2Bn32Jio=",
                      "clientNumber": "ZdUy3iRki2Sh4qovU9loFsic8/Y+YtjWJEdEhAXKsA8=",
                      "companyClass": "RT",
                      "companyName": "MARKS & SPENCER FIN SERV PLC (C) (I)",
                      "currentBalance": {
                        "balanceAmount": { "amount": 1472, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 11, "month": 5, "year": 1985 },
                      "fixedPaymentTerms": {
                        "numberOfPayments": 48,
                        "paymentAmount": { "amount": 210, "currency": "GBP" }
                      },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 3, "month": 9, "year": 2023 },
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 1472, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 1682, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 1892, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 2103, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 2313, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 2523, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 2734, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 2944, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 3154, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 3364, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 3575, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 3785, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 3995, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 4206, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 4416, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 4626, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 15,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 4837, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 16,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 5047, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 17,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 5257, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 18,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 5468, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 19,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 5678, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 20,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 5888, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 21,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 6098, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 22,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 6309, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 23,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 6519, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 24,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 6729, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 25,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 6940, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 26,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 7150, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 27,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 7360, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 28,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 7571, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 29,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 7781, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 30,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 7991, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 31,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 8202, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 32,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 8412, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 33,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 8622, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 34,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 8833, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 35,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 9043, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 36,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 9253, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 37,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 9463, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 38,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 9674, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 39,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 9884, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 40,
                          "paymentStatus": "U"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 10094, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 41,
                          "paymentStatus": "U"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 19, "month": 3, "year": 2020 }
                    },
                    {
                      "accountNumber": "tSPyQ+mZayou0q6iQiI3680Jg1GD3aCNuzxB8ph2wX8=",
                      "clientNumber": "7n6dgI3zy+vxzsLawG4LAaINtX07x8h/G8zRHoqkos4=",
                      "companyClass": "BK",
                      "companyName": "MBNA LOANS (I)",
                      "currentBalance": {
                        "balanceAmount": { "amount": 11293, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 9, "month": 12, "year": 1985 },
                      "fixedPaymentTerms": {
                        "numberOfPayments": 60,
                        "paymentAmount": { "amount": 227, "currency": "GBP" }
                      },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 14, "month": 9, "year": 2023 },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 11293, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 11470, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 11648, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 11823, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 12000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "paymentStatus": "ZERO"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 1, "month": 4, "year": 2023 }
                    }
                  ],
                  "xmasClub": []
                },
                "previousSearches": {
                  "previousCreditSearch": [
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 24, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 15, "month": 3, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 3, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 10, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 6, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 13, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 26, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 13, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 27, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 20, "month": 10, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 13, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 15, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 12, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 15, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 12, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 7, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 7, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 22, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 8, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 1, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 8, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 16, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 14, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 8, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 12, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 13, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 27, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 21, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 27, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 26, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 17, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 26, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 2, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 4, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 16, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 12, "month": 6, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 2, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 20, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 5, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                      "companyName": "BLUE MOTOR FINANCE LTD",
                      "companyType": "FS",
                      "dob": { "day": 12, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 9, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 3, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 6, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 2, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 3, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 4, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 24, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 15, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 5, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 7, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 2, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 7, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 13, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 1, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 3, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "pZApBJ6w6t9j5KWbFbmPfpT+xYddRw1rh+NBX2UrPHo=",
                      "companyName": "HASTINGS LOANS",
                      "companyType": "IN",
                      "dob": { "day": 26, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 22, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                      "companyName": "BLUE MOTOR FINANCE LTD",
                      "companyType": "FS",
                      "dob": { "day": 9, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 12, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 18, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 22, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 11, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 26, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 10, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 24, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 16, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 2, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 3, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 9, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 3, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 8, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 3, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 4, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 24, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 16, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 21, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 21, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 31, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 21, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 31, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 20, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 31, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 14, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 31, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 22, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 31, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 4, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 31, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 21, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 5, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 1, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 21, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 19, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 21, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 4, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 26, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 20, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 25, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 13, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 20, "month": 3, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 2, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 11, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 4, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 4, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 19, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 4, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 19, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 7, "month": 10, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 23, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 7, "month": 10, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "1sHKig3Hx4VDfgLymcLtLdje7l90irsRHH2hP1fjszM=",
                      "companyName": "CASHPLUS",
                      "companyType": "FS",
                      "dob": { "day": 6, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 27, "month": 9, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 24, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 27, "month": 9, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 26, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 27, "month": 9, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 22, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 1, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 5, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 22, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 9, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 25, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                      "companyName": "BLUE MOTOR FINANCE LTD",
                      "companyType": "FS",
                      "dob": { "day": 18, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 21, "month": 3, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 3, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 12, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 20, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 15, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                      "companyName": "BLUE MOTOR FINANCE LTD",
                      "companyType": "FS",
                      "dob": { "day": 9, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 1, "month": 3, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                      "companyName": "BLUE MOTOR FINANCE LTD",
                      "companyType": "FS",
                      "dob": { "day": 26, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    }
                  ],
                  "previousNonCreditSearch": [
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 18, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 21, "month": 7, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 12, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 15, "month": 9, "year": 2023 },
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 26, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 7, "month": 7, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 27, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 2, "month": 9, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 16, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 9, "month": 9, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 16, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 26, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 17, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 14, "month": 7, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 24, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 25, "month": 8, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 21, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 16, "month": 9, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 22, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 19, "month": 11, "year": 2022 },
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 3, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 4, "month": 9, "year": 2023 },
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 2, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 11, "month": 8, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 5, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 4, "month": 8, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 16, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 21, "month": 3, "year": 2023 },
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 24, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 18, "month": 8, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 4, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 30, "month": 6, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 6, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 28, "month": 7, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 18, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 1, "month": 3, "year": 2023 },
                      "sourcedFrom": "ASY"
                    }
                  ]
                },
                "rollingRegisterData": {
                  "rollingRegister": [
                    {
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU"
                      },
                      "nameMatchStatus": "A",
                      "recordType": "LOAD",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "RRR",
                      "supplyDate": "092022"
                    },
                    {
                      "name": { "forename": "GIHOY", "surname": "HENYJACI" },
                      "nameMatchStatus": "C",
                      "recordType": "LOAD",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "RRR",
                      "supplyDate": "092022"
                    }
                  ]
                }
              },
              "index": 1,
              "matchedAddress": {
                "address": {
                  "addressID": "pzpU8pUJMIOtRgSUqEVWj527/cmIn0nErTB63NuHjkw=",
                  "county": "PIQW GYHZIF",
                  "number": "25447",
                  "postTown": "HORSHAM",
                  "postcode": "SE11 5JA",
                  "street1": "LZOQYQFI GYYW"
                },
                "sourcedFrom": "ADO"
              },
              "noticeOfCorrectionOrDisputePresent": false,
              "potentialMatchedAddress": []
            },
            {
              "addressMatchStatus": "SINGLE_MATCH",
              "addressSpecificData": {
                "electoralRollData": {
                  "electoralRoll": [
                    {
                      "annualRegisterPeriod": { "end": "2023", "start": "2021" },
                      "name": {
                        "forename": "LETONR",
                        "middleName": "SOPUX",
                        "surname": "PUIQQBEPN"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2022", "start": "2020" },
                      "name": { "forename": "GIHOY", "surname": "HENYJACI" },
                      "nameMatchStatus": "C",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2022", "start": "2017" },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "HUSOOT",
                        "surname": "XYHKUHXOXU"
                      },
                      "nameMatchStatus": "A",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2017", "start": "2014" },
                      "name": {
                        "forename": "ZEGWEC",
                        "middleName": "DODL",
                        "surname": "FUZIK"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2017", "start": "2014" },
                      "name": {
                        "forename": "QUUZUO",
                        "middleName": "VUESXU",
                        "surname": "KYDPOG"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "2014", "start": "1989" },
                      "name": { "forename": "BYPSDUUP", "surname": "HUPPUZY" },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "1985", "start": "1983" },
                      "name": {
                        "forename": "UMU",
                        "middleName": "W",
                        "surname": "NUCT",
                        "title": "MS"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "1989", "start": "1986" },
                      "name": {
                        "forename": "ICTATS",
                        "middleName": "N",
                        "surname": "VKWYKKIKS"
                      },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    },
                    {
                      "annualRegisterPeriod": { "end": "1989", "start": "1986" },
                      "name": { "forename": "LOSI", "surname": "VKWYKKIKS" },
                      "nameMatchStatus": "Z",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "ELR"
                    }
                  ]
                },
                "insightData": {
                  "bankDefaultAgreement": [],
                  "basicBankAccount": [],
                  "bridgingFinance": [],
                  "budgetAccount": [],
                  "buyToLetMortgage": [],
                  "chargeCard": [],
                  "commsSupplyAccount": [],
                  "consolidatedDebt": [],
                  "councilArrears": [],
                  "creditCard": [
                    {
                      "accountNumber": "cuDI8n9lXiaFB7MaVKIjNWqsNLVPhMTrHEKIe6iez1U=",
                      "clientNumber": "HHVTalQAH9CYjX8vPtUOn5FZPCbqgg+zH0/PjZD93xY=",
                      "companyClass": "FS",
                      "companyName": "LLOYDS BANK (WAS LLOYDS TSB) (I)",
                      "creditLimit": {
                        "limit": { "amount": 10000, "currency": "GBP" },
                        "suppressed": false
                      },
                      "currentBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 18, "month": 9, "year": 1985 },
                      "endDate": { "day": 2, "month": 5, "year": 2018 },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 18, "month": 6, "year": 2018 },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "S",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 15,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 16,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 17,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 18,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 19,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 20,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 21,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 22,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 23,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 24,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 25,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 26,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "creditLimitChange": "NO_CHANGE",
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 27,
                          "creditLimit": {
                            "limit": { "amount": 10000, "currency": "GBP" },
                            "suppressed": false
                          },
                          "paymentStatus": "ZERO",
                          "statement": {
                            "cashAdvanceCount": 0,
                            "cashAdvanceValue": { "amount": 0, "currency": "GBP" },
                            "paymentAmount": { "amount": 0, "currency": "GBP" },
                            "statementBalance": {
                              "balanceAmount": { "amount": 0, "currency": "GBP" },
                              "suppressed": false
                            }
                          }
                        }
                      ],
                      "sourcedFrom": "INY,IBR,ICR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 2, "month": 2, "year": 2016 }
                    }
                  ],
                  "currentAccount": [],
                  "fixedTermAgreement": [],
                  "greenDeals": [],
                  "hirePurchase": [],
                  "homeLendingAgreement": [],
                  "insuranceAgreement": [],
                  "localAuthorityHousing": [],
                  "mailOrderAccount": [],
                  "optionAccount": [],
                  "payDayOrShortTermLoan": [],
                  "propertyRental": [],
                  "publicUtilityAccount": [],
                  "rentalAgreement": [],
                  "securedLoan": [
                    {
                      "accountNumber": "r9jjexGpGIiqxQJx1AODd+N2KFtABRCSglQNZ26UguE=",
                      "clientNumber": "F6TtkHgZMA0k4thIonIcjUfIoOqmln/6oOm4L8BcRfY=",
                      "companyClass": "MS",
                      "companyName": "Lloyds Bank Mortgages Plc",
                      "currentBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 2, "month": 7, "year": 1985 },
                      "endDate": { "day": 10, "month": 6, "year": 2022 },
                      "fixedPaymentTerms": {
                        "numberOfPayments": 0,
                        "paymentAmount": { "amount": 1641, "currency": "GBP" }
                      },
                      "flexible": false,
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 13, "month": 7, "year": 2022 },
                      "loanType": "MORTGAGE",
                      "name": {
                        "forename": "OPIC",
                        "middleName": "K",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "paymentStatus": "S"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 268647,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 269682,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 270735,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 271766,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 272853,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 273879,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 274903,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 7,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 275944,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 8,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 276963,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 9,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 278000,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 10,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 279014,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 11,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 280026,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 12,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 281056,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 13,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 282064,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 14,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 283089,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 15,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 284092,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 16,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 285155,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 17,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 286153,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 18,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 287151,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 19,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 288167,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 20,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 289161,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 21,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 290173,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 22,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 291162,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 23,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 292148,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 24,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 293154,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 25,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 294136,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 26,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 295137,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 27,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 296115,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 28,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 297133,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 29,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 298106,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 30,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 299075,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 31,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 300064,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 32,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 301029,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 33,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 302013,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 34,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 302974,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 35,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 303932,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 36,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 304910,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 37,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 305771,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 38,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 306657,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 39,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 307513,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 40,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 308452,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 41,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 309303,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 42,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 310151,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 43,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 311025,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 44,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 311868,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 45,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 312737,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 46,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": {
                              "amount": 313575,
                              "currency": "GBP"
                            },
                            "suppressed": false
                          },
                          "ageInMonths": 47,
                          "paymentStatus": "ZERO"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 30, "month": 6, "year": 2017 }
                    }
                  ],
                  "socialHousingRental": [],
                  "studentLoan": [],
                  "uncategorisedAgreement": [],
                  "unpresentableCheque": [],
                  "unsecuredLoan": [
                    {
                      "accountNumber": "lsrZ9iCAvJoLUh3DsLmI8ynvqpAUObNt/CHTC7WZGrc=",
                      "clientNumber": "uiNiWE7hUanszyt4z0um7++3Kg4tFTGtCxF19YvjQFs=",
                      "companyClass": "FN",
                      "companyName": "SHAWBROOK BANK (I)",
                      "currentBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "defaultBalance": {
                        "balanceAmount": { "amount": 0, "currency": "GBP" },
                        "suppressed": false
                      },
                      "dob": { "day": 3, "month": 6, "year": 1985 },
                      "endDate": { "day": 9, "month": 10, "year": 2018 },
                      "fixedPaymentTerms": {
                        "numberOfPayments": 6,
                        "paymentAmount": { "amount": 320, "currency": "GBP" }
                      },
                      "insightQuality": {
                        "qualityIndicator1": false,
                        "qualityIndicator2": false
                      },
                      "lastUpdateDate": { "day": 5, "month": 11, "year": 2018 },
                      "name": {
                        "forename": "OPIC",
                        "middleName": "HUSOOT",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "paymentFrequency": "MONTHLY",
                      "paymentHistory": [
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 0, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 0,
                          "paymentStatus": "S"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 320, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 1,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 640, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 2,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 960, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 3,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 1280, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 4,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 1600, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 5,
                          "paymentStatus": "ZERO"
                        },
                        {
                          "accountBalance": {
                            "balanceAmount": { "amount": 1920, "currency": "GBP" },
                            "suppressed": false
                          },
                          "ageInMonths": 6,
                          "paymentStatus": "U"
                        }
                      ],
                      "sourcedFrom": "INY,IBR",
                      "startBalance": {
                        "balanceAmount": { "amount": 1920, "currency": "GBP" },
                        "suppressed": false
                      },
                      "startDate": { "day": 9, "month": 4, "year": 2018 }
                    }
                  ],
                  "xmasClub": []
                },
                "previousSearches": {
                  "previousCreditSearch": [
                    {
                      "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                      "companyName": "OAKBROOK FINANCE LTD",
                      "companyType": "FN",
                      "dob": { "day": 8, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 7, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "uewaX1KDZA8zwtmECKtjOkSq1E8eO7CfysLRASlIufE=",
                      "companyName": "LIVELEND",
                      "companyType": "FS",
                      "dob": { "day": 27, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "dlRXBa74WZT5EJfsoQaPd8rsktxoAdXE6Llor2roEO8=",
                      "companyName": "AMIGO LOANS LTD T/A REWARDRATE",
                      "companyType": "MN",
                      "dob": { "day": 9, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "uewaX1KDZA8zwtmECKtjOkSq1E8eO7CfysLRASlIufE=",
                      "companyName": "LIVELEND",
                      "companyType": "FS",
                      "dob": { "day": 26, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                      "companyName": "OAKBROOK FINANCE LTD",
                      "companyType": "FN",
                      "dob": { "day": 6, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                      "companyName": "ZOPA CARD WITH CLEARSCORE",
                      "companyType": "MN",
                      "dob": { "day": 21, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                      "companyName": "ZOPA CARD WITH CLEARSCORE",
                      "companyType": "MN",
                      "dob": { "day": 17, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 8, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                      "companyName": "ZOPA CARD WITH CLEARSCORE",
                      "companyType": "MN",
                      "dob": { "day": 13, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 4, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                      "companyName": "OAKBROOK FINANCE LTD",
                      "companyType": "FN",
                      "dob": { "day": 4, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 3, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                      "companyName": "OAKBROOK FINANCE LTD",
                      "companyType": "FN",
                      "dob": { "day": 8, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 1, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                      "companyName": "ZOPA CARD WITH CLEARSCORE",
                      "companyType": "MN",
                      "dob": { "day": 14, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                      "companyName": "OAKBROOK FINANCE LTD",
                      "companyType": "FN",
                      "dob": { "day": 13, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 30, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                      "companyName": "ZOPA CARD WITH CLEARSCORE",
                      "companyType": "MN",
                      "dob": { "day": 23, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 15, "month": 3, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "4eYHxdool6EiXG14DnYX8oq3j/C81YO9BN76p1Fr2hU=",
                      "companyName": "ZOPA CARD WITH CLEARSCORE",
                      "companyType": "MN",
                      "dob": { "day": 6, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                      "companyName": "OAKBROOK FINANCE LTD",
                      "companyType": "FN",
                      "dob": { "day": 26, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "uewaX1KDZA8zwtmECKtjOkSq1E8eO7CfysLRASlIufE=",
                      "companyName": "LIVELEND",
                      "companyType": "FS",
                      "dob": { "day": 16, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 7, "month": 10, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "DBA+onnOA6G0A+vX8XIRqh99D5beSJKWgvY9YqGQej8=",
                      "companyName": "OAKBROOK FINANCE LTD",
                      "companyType": "FN",
                      "dob": { "day": 6, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 10, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 21, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 5, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 5, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                      "companyName": "BLUE MOTOR FINANCE LTD",
                      "companyType": "FS",
                      "dob": { "day": 9, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 1, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 7, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 18, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 20, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 22, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 8, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 25, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 7, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 8, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 20, "month": 10, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 24, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 27, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 18, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 28, "month": 11, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 21, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 24, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 17, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 11, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 25, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 4, "month": 5, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "H63sJEAAHmXxdsYp/ybHdm21RvR3zlHo8jJU1pBOxZQ=",
                      "companyName": "HASTINGS FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 13, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 15, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 15, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 15, "month": 9, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 4, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 14, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 23, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 9, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 23, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 3, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 27, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 8, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 18, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 15, "month": 12, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 2, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 1, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 23, "month": 8, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 20, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 19, "month": 1, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "oKwfQ5jmuI7xBod/InJZA16/Bhq3hZrpXKKEbTOi25I=",
                      "companyName": "BLUE MOTOR FINANCE LTD",
                      "companyType": "FS",
                      "dob": { "day": 26, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 21, "month": 3, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 15, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 1, "month": 4, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 12, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 1, "month": 4, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 6, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 1, "month": 4, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "9Xw3i9mKKjwNl6YAmlRIrlhoYPSnOkOntYP/Z7iqIXY=",
                      "companyName": "ADMIRAL FINANCIAL SERVICES LTD",
                      "companyType": "FS",
                      "dob": { "day": 2, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 1, "month": 4, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 17, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 18, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 5, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 4, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 20, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 17, "month": 2, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 27, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 27, "month": 9, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 3, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 27, "month": 9, "year": 2022 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "wzBJzuGHVtz+MtKQjLlghU/vsQo5hsyIpEZ50G9hfnQ=",
                      "companyName": "CLEARSCORE WITH CAPITAL ONE",
                      "companyType": "CC",
                      "dob": { "day": 4, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 31, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+3EG5jN2Ji6o0Di6NNUL36t2vsN/cot4qfgKiIJQbh8=",
                      "companyName": "BARCLAYCARD",
                      "companyType": "BK",
                      "dob": { "day": 20, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "optIn": true,
                      "searchDate": { "day": 31, "month": 7, "year": 2023 },
                      "searchType": "CREDIT_QUOTATION",
                      "sourcedFrom": "ASY"
                    }
                  ],
                  "previousNonCreditSearch": [
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 27, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 27, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 18, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 23, "month": 2, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 15, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 23, "month": 2, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 25, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 8, "month": 2, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 15, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 8, "month": 2, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "GO5LJLy0r2dTBuNHnUCrGp69LCieQ6g0xeC6q5eTg+4=",
                      "companyName": "HASTINGS INSURANCE SERVICES LTD",
                      "companyType": "IN",
                      "dob": { "day": 2, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 21, "month": 5, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "PvD41LwoaR1LHyepd3DgioyzWQL4hmVlu3EkxnX+4Zk=",
                      "companyName": "EUI LIMITED T/A ADMIRAL",
                      "companyType": "IN",
                      "dob": { "day": 17, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 21, "month": 5, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "v1yCvhOwDzGXEhp+r8KoqiZAkhybVeDV60z1ke6ftD0=",
                      "companyName": "PERCAYSO-INFORM.CO.UK",
                      "companyType": "IN",
                      "dob": { "day": 11, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 21, "month": 5, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 9, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 25, "month": 3, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 2, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 25, "month": 3, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 22, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 21, "month": 3, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 11, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 21, "month": 3, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "weLdQbAYgKf84WCLqmE+OSZP6Rof2a3OV97D9/GhfVI=",
                      "companyName": "HASTINGS INSURANCE SERVICES LTD",
                      "companyType": "IN",
                      "dob": { "day": 20, "month": 5, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 1, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 24, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 17, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 9, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 17, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 23, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 17, "month": 2, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 8, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 17, "month": 2, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 21, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 17, "month": 2, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 11, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 17, "month": 2, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "weLdQbAYgKf84WCLqmE+OSZP6Rof2a3OV97D9/GhfVI=",
                      "companyName": "HASTINGS INSURANCE SERVICES LTD",
                      "companyType": "IN",
                      "dob": { "day": 22, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 5, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 25, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 19, "month": 11, "year": 2022 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 22, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 19, "month": 11, "year": 2022 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 2, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 15, "month": 3, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 5, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 15, "month": 3, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 13, "month": 1, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 19, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 15, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 19, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 18, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 7, "month": 10, "year": 2022 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "+C3ZtofczpBaZsjsMLGs6p9z1npZrueilnQ4UXE3j8A=",
                      "companyName": "CREDITSPRING",
                      "companyType": "FS",
                      "dob": { "day": 27, "month": 8, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 7, "month": 10, "year": 2022 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 4, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 7, "month": 7, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 2, "month": 3, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 8, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 28, "month": 12, "year": 2022 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 20, "month": 2, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 28, "month": 7, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 17, "month": 11, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 15, "month": 9, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 26, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 30, "month": 6, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 3, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 4, "month": 8, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "06lWu16dJ5BmFWsJTl8ZRBvV2Umli03GgeOx+VGZfqI=",
                      "companyName": "CLEARSCORE TECHNOLOGY LTD",
                      "companyType": "CZ",
                      "dob": { "day": 24, "month": 4, "year": 1985 },
                      "jointApplicant": false,
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 2, "month": 9, "year": 2023 },
                      "searchType": "CREDITFILE_REQUEST",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 22, "month": 9, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 21, "month": 3, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 25, "month": 7, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 1, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 25, "month": 12, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 1, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    },
                    {
                      "accountNumber": "t9EKwQw6aKlC9uFzxwWWcorgv6cItdWiBiyEr5OaHaQ=",
                      "companyName": "ZOPA",
                      "companyType": "FN",
                      "dob": { "day": 19, "month": 6, "year": 1985 },
                      "jointApplicant": false,
                      "name": {
                        "forename": "OPIC",
                        "surname": "XYHKUHXOXU",
                        "title": "MR"
                      },
                      "nameMatchStatus": "A",
                      "searchDate": { "day": 1, "month": 4, "year": 2023 },
                      "searchType": "NO_SEARCH_TYPE",
                      "sourcedFrom": "ASY"
                    }
                  ]
                },
                "rollingRegisterData": {
                  "rollingRegister": [
                    {
                      "name": {
                        "forename": "LETONR",
                        "middleName": "SOPUX",
                        "surname": "PUIQQBEPN"
                      },
                      "nameMatchStatus": "Z",
                      "recordType": "LOAD",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "RRR",
                      "supplyDate": "082022"
                    },
                    {
                      "name": { "forename": "GIHOY", "surname": "HENYJACI" },
                      "nameMatchStatus": "C",
                      "recordType": "DELETE",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "RRR",
                      "supplyDate": "092022"
                    },
                    {
                      "name": {
                        "forename": "OPIC",
                        "middleName": "HUSOOT",
                        "surname": "XYHKUHXOXU"
                      },
                      "nameMatchStatus": "A",
                      "recordType": "DELETE",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "RRR",
                      "supplyDate": "092022"
                    },
                    {
                      "name": { "forename": "OPIC", "surname": "XYHKUHXOXU" },
                      "nameMatchStatus": "A",
                      "recordType": "LOAD",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "RRR",
                      "supplyDate": "082017"
                    },
                    {
                      "name": { "forename": "QUUZUO", "surname": "KYDPOG" },
                      "nameMatchStatus": "Z",
                      "recordType": "DELETE",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "RRR",
                      "supplyDate": "072017"
                    },
                    {
                      "name": { "forename": "BYPSDUUP", "surname": "HUPPUZY" },
                      "nameMatchStatus": "Z",
                      "recordType": "DELETE",
                      "seniority": "UNKNOWN",
                      "sourcedFrom": "RRR",
                      "supplyDate": "092014"
                    }
                  ]
                }
              },
              "index": 2,
              "matchedAddress": {
                "address": {
                  "addressID": "KjcDX9zzilJxw6RMACDldGzn22Pt5t4onuIknPoHOBg=",
                  "county": "RIJMX",
                  "number": "97363",
                  "postTown": "WATFORD",
                  "postcode": "SE11 5JA",
                  "street1": "BAVMYV SUYT"
                },
                "sourcedFrom": "ADO"
              },
              "noticeOfCorrectionOrDisputePresent": false,
              "potentialMatchedAddress": []
            }
          ]
        }
      }
    }
    
  "#;
    report.to_string()
}

pub fn get_parsed_reports() -> Reports {
    parse_reports(vec![get_latest_report(), get_historical_report()])
}

#[allow(dead_code)]
pub fn run_graphql_query(query: &str, variables: HashMap<String, InputValue>) -> juniper::Value {
    let schema = create_schema();
    let context = Context {
        reports: get_parsed_reports(),
    };

    juniper::execute_sync(query, None, &schema, &variables, &context)
        .unwrap()
        .0
}
