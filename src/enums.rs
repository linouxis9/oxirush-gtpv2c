/*
   OxiRush
   Copyright 2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! The values that the tables of clause 8 name, one enumeration each.

oxirush_utils::named_values! {
    /// Cause values (Table 8.4-1). A value that "shall not be used", of earlier versions,
    /// and a spare value of a response are read as "Request rejected (reason not
    /// specified)" (NOTE 3, NOTE 4).
    pub enum CauseValue "Table 8.4-1" bits 8 reserved [0] read [71 ..= 71 as 94, 79 ..= 79 as 94, 99 ..= 99 as 94, 118 ..= 118 as 94, 132 ..= 239 as 94] {
        2 LocalDetach "Local Detach",
        3 CompleteDetach "Complete Detach",
        4 RatChangedFrom3gppToNon3gpp "RAT changed from 3GPP to Non-3GPP",
        5 IsrDeactivation "ISR deactivation",
        6 ErrorIndicationReceived "Error Indication received from RNC/eNodeB/S4-SGSN/MME",
        7 ImsiDetachOnly "IMSI Detach Only",
        8 ReactivationRequested "Reactivation Requested",
        9 PdnReconnectionToThisApnDisallowed "PDN reconnection to this APN disallowed",
        10 AccessChangedFromNon3gppTo3gpp "Access changed from Non-3GPP to 3GPP",
        11 PdnConnectionInactivityTimerExpires "PDN connection inactivity timer expires",
        12 PgwNotResponding "PGW not responding",
        13 NetworkFailure "Network Failure",
        14 QosParameterMismatch "QoS parameter mismatch",
        15 EpsTo5gsMobility "EPS to 5GS Mobility",
        16 RequestAccepted "Request accepted",
        17 RequestAcceptedPartially "Request accepted partially",
        18 NewPdnTypeDueToNetworkPreference "New PDN type due to network preference",
        19 NewPdnTypeDueToSingleAddressBearerOnly "New PDN type due to single address bearer only",
        64 ContextNotFound "Context Not Found",
        65 InvalidMessageFormat "Invalid Message Format",
        66 VersionNotSupportedByNextPeer "Version not supported by next peer",
        67 InvalidLength "Invalid length",
        68 ServiceNotSupported "Service not supported",
        69 MandatoryIeIncorrect "Mandatory IE incorrect",
        70 MandatoryIeMissing "Mandatory IE missing",
        72 SystemFailure "System failure",
        73 NoResourcesAvailable "No resources available",
        74 SemanticErrorInTheTftOperation "Semantic error in the TFT operation",
        75 SyntacticErrorInTheTftOperation "Syntactic error in the TFT operation",
        76 SemanticErrorsInPacketFilters "Semantic errors in packet filter(s)",
        77 SyntacticErrorsInPacketFilters "Syntactic errors in packet filter(s)",
        78 MissingOrUnknownApn "Missing or unknown APN",
        80 GreKeyNotFound "GRE key not found",
        81 RelocationFailure "Relocation failure",
        82 DeniedInRat "Denied in RAT",
        83 PreferredPdnTypeNotSupported "Preferred PDN type not supported",
        84 AllDynamicAddressesAreOccupied "All dynamic addresses are occupied",
        85 UeContextWithoutTftAlreadyActivated "UE context without TFT already activated",
        86 ProtocolTypeNotSupported "Protocol type not supported",
        87 UeNotResponding "UE not responding",
        88 UeRefuses "UE refuses",
        89 ServiceDenied "Service denied",
        90 UnableToPageUe "Unable to page UE",
        91 NoMemoryAvailable "No memory available",
        92 UserAuthenticationFailed "User authentication failed",
        93 ApnAccessDeniedNoSubscription "APN access denied – no subscription",
        94 RequestRejected "Request rejected (reason not specified)",
        95 PTmsiSignatureMismatch "P-TMSI Signature mismatch",
        96 ImsiImeiNotKnown "IMSI/IMEI not known",
        97 SemanticErrorInTheTadOperation "Semantic error in the TAD operation",
        98 SyntacticErrorInTheTadOperation "Syntactic error in the TAD operation",
        100 RemotePeerNotResponding "Remote peer not responding",
        101 CollisionWithNetworkInitiatedRequest "Collision with network initiated request",
        102 UnableToPageUeDueToSuspension "Unable to page UE due to Suspension",
        103 ConditionalIeMissing "Conditional IE missing",
        104 ApnRestrictionTypeIncompatible "APN Restriction type Incompatible with currently active PDN connection",
        105 InvalidOverallLength "Invalid overall length of the triggered response message and a piggybacked initial message",
        106 DataForwardingNotSupported "Data forwarding not supported",
        107 InvalidReplyFromRemotePeer "Invalid reply from remote peer",
        108 FallbackToGtpv1 "Fallback to GTPv1",
        109 InvalidPeer "Invalid peer",
        110 TemporarilyRejectedDueToMobility "Temporarily rejected due to handover/TAU/RAU procedure in progress",
        111 ModificationsNotLimitedToS1uBearers "Modifications not limited to S1-U bearers",
        112 RequestRejectedForAPmipv6Reason "Request rejected for a PMIPv6 reason",
        113 ApnCongestion "APN Congestion",
        114 BearerHandlingNotSupported "Bearer handling not supported",
        115 UeAlreadyReAttached "UE already re-attached",
        116 MultiplePdnConnectionsForAGivenApnNotAllowed "Multiple PDN connections for a given APN not allowed",
        117 TargetAccessRestrictedForTheSubscriber "Target access restricted for the subscriber",
        119 MmeSgsnRefusesDueToVplmnPolicy "MME/SGSN refuses due to VPLMN Policy",
        120 GtpCEntityCongestion "GTP-C Entity Congestion",
        121 LateOverlappingRequest "Late Overlapping Request",
        122 TimedOutRequest "Timed out Request",
        123 UeIsTemporarilyNotReachableDueToPowerSaving "UE is temporarily not reachable due to power saving",
        124 RelocationFailureDueToNasMessageRedirection "Relocation failure due to NAS message redirection",
        125 UeNotAuthorisedByOcsOrExternalAaaServer "UE not authorised by OCS or external AAA Server",
        126 MultipleAccessesToAPdnConnectionNotAllowed "Multiple accesses to a PDN connection not allowed",
        127 RequestRejectedDueToUeCapability "Request rejected due to UE capability",
        128 S1uPathFailure "S1-U Path Failure",
        129 FiveGcNotAllowed "5GC not allowed",
        130 PgwMismatchWithNetworkSliceSubscribedByTheUe "PGW mismatch with network slice subscribed by the UE",
        131 RejectionDueToPagingRestriction "Rejection due to paging restriction",
    }
}

impl CauseValue {
    /// Whether a response of this Cause rejects its request: 64 to 239 (Table 8.4-1).
    pub fn rejects(self) -> bool {
        (64..=239).contains(&self.value())
    }
}

oxirush_utils::named_values! {
    /// RAT Type (Table 8.17-1).
    pub enum RatTypeValue "Table 8.17-1" bits 8 reserved [0] {
        1 Utran "UTRAN",
        2 Geran "GERAN",
        3 Wlan "WLAN",
        4 Gan "GAN",
        5 HspaEvolution "HSPA Evolution",
        6 Eutran "EUTRAN (WB-E-UTRAN)",
        7 Virtual "Virtual",
        8 EutranNbIot "EUTRAN-NB-IoT",
        9 LteM "LTE-M",
        10 Nr "NR",
        11 WbEutranLeo "WB-E-UTRAN(LEO)",
        12 WbEutranMeo "WB-E-UTRAN(MEO)",
        13 WbEutranGeo "WB-E-UTRAN(GEO)",
        14 WbEutranOthersat "WB-E-UTRAN(OTHERSAT)",
        15 EutranNbIotLeo "EUTRAN-NB-IoT(LEO)",
        16 EutranNbIotMeo "EUTRAN-NB-IoT(MEO)",
        17 EutranNbIotGeo "EUTRAN-NB-IoT(GEO)",
        18 EutranNbIotOthersat "EUTRAN-NB-IoT(OTHERSAT)",
        19 LteMLeo "LTE-M(LEO)",
        20 LteMMeo "LTE-M(MEO)",
        21 LteMGeo "LTE-M(GEO)",
        22 LteMOthersat "LTE-M(OTHERSAT)",
    }
    /// Interface Type of an F-TEID (§8.22).
    pub enum InterfaceType "8.22" bits 6 {
        0 S1uEnodebGtpU "S1-U eNodeB GTP-U interface",
        1 S1uSgwGtpU "S1-U SGW GTP-U interface",
        2 S12RncGtpU "S12 RNC GTP-U interface",
        3 S12SgwGtpU "S12 SGW GTP-U interface",
        4 S5S8SgwGtpU "S5/S8 SGW GTP-U interface",
        5 S5S8PgwGtpU "S5/S8 PGW GTP-U interface",
        6 S5S8SgwGtpC "S5/S8 SGW GTP-C interface",
        7 S5S8PgwGtpC "S5/S8 PGW GTP-C interface",
        8 S5S8SgwPmipv6 "S5/S8 SGW PMIPv6 interface",
        9 S5S8PgwPmipv6 "S5/S8 PGW PMIPv6 interface",
        10 S11MmeGtpC "S11 MME GTP-C interface",
        11 S11S4SgwGtpC "S11/S4 SGW GTP-C interface",
        12 S10N26MmeGtpC "S10/N26 MME GTP-C interface",
        13 S3MmeGtpC "S3 MME GTP-C interface",
        14 S3SgsnGtpC "S3 SGSN GTP-C interface",
        15 S4SgsnGtpU "S4 SGSN GTP-U interface",
        16 S4SgwGtpU "S4 SGW GTP-U interface",
        17 S4SgsnGtpC "S4 SGSN GTP-C interface",
        18 S16SgsnGtpC "S16 SGSN GTP-C interface",
        19 EnodebGnodebGtpUDlForwarding "eNodeB/gNodeB GTP-U interface for DL data forwarding",
        20 EnodebGtpUUlForwarding "eNodeB GTP-U interface for UL data forwarding",
        21 RncGtpUForwarding "RNC GTP-U interface for data forwarding",
        22 SgsnGtpUForwarding "SGSN GTP-U interface for data forwarding",
        23 SgwUpfGtpUDlForwarding "SGW/UPF GTP-U interface for DL data forwarding",
        24 SmMbmsGwGtpC "Sm MBMS GW GTP-C interface",
        25 SnMbmsGwGtpC "Sn MBMS GW GTP-C interface",
        26 SmMmeGtpC "Sm MME GTP-C interface",
        27 SnSgsnGtpC "Sn SGSN GTP-C interface",
        28 SgwGtpUUlForwarding "SGW GTP-U interface for UL data forwarding",
        29 SnSgsnGtpU "Sn SGSN GTP-U interface",
        30 S2bEpdgGtpC "S2b ePDG GTP-C interface",
        31 S2bUEpdgGtpU "S2b-U ePDG GTP-U interface",
        32 S2bPgwGtpC "S2b PGW GTP-C interface",
        33 S2bUPgwGtpU "S2b-U PGW GTP-U interface",
        34 S2aTwanGtpU "S2a TWAN GTP-U interface",
        35 S2aTwanGtpC "S2a TWAN GTP-C interface",
        36 S2aPgwGtpC "S2a PGW GTP-C interface",
        37 S2aPgwGtpU "S2a PGW GTP-U interface",
        38 S11MmeGtpU "S11 MME GTP-U interface",
        39 S11SgwGtpU "S11 SGW GTP-U interface",
        40 N26AmfGtpC "N26 AMF GTP-C interface",
        41 N19mbUpfGtpU "N19mb UPF GTP-U interface",
    }
    /// PDN Type (§8.34, Table 8.14-1).
    pub enum PdnTypeValue "8.34" bits 3 {
        1 Ipv4 "IPv4",
        2 Ipv6 "IPv6",
        3 Ipv4v6 "IPv4v6",
        4 NonIp "Non-IP",
        5 Ethernet "Ethernet",
    }
    /// Selection Mode (Table 8.58-1): 3, of a later use, is read as 2.
    pub enum SelectionModeValue "Table 8.58-1" bits 2 read [3 ..= 3 as 2] {
        0 MsOrNetworkProvidedApnSubscriptionVerified "MS or network provided APN, subscription verified",
        1 MsProvidedApnSubscriptionNotVerified "MS provided APN, subscription not verified",
        2 NetworkProvidedApnSubscriptionNotVerified "Network provided APN, subscription not verified",
    }
    /// Security Mode of an MM Context (Table 8.38-1).
    pub enum SecurityMode "Table 8.38-1" bits 3 {
        0 GsmKeyAndTriplets "GSM Key and Triplets",
        1 UmtsKeyUsedCipherAndQuintuplets "UMTS Key, Used Cipher and Quintuplets",
        2 GsmKeyUsedCipherAndQuintuplets "GSM Key, Used Cipher and Quintuplets",
        3 UmtsKeyAndQuintuplets "UMTS Key and Quintuplets",
        4 EpsSecurityContextAndQuadruplets "EPS Security Context and Quadruplets",
        5 UmtsKeyQuadrupletsAndQuintuplets "UMTS Key, Quadruplets and Quintuplets",
    }
    /// Used NAS integrity protection algorithm of an MM Context (Table 8.38-4).
    pub enum NasIntegrity "Table 8.38-4" bits 3 {
        0 None "No integrity protection",
        1 Eia1 "128-EIA1",
        2 Eia2 "128-EIA2",
        3 Eia3 "128-EIA3",
        4 Eia4 "EIA4",
        5 Eia5 "EIA5",
        6 Eia6 "EIA6",
        7 Eia7 "EIA7",
    }
    /// Used NAS Cipher of an MM Context (Table 8.38-2).
    pub enum NasCipher "Table 8.38-2" bits 4 {
        0 None "No ciphering",
        1 Eea1 "128-EEA1",
        2 Eea2 "128-EEA2",
        3 Eea3 "128-EEA3",
        4 Eea4 "EEA4",
        5 Eea5 "EEA5",
        6 Eea6 "EEA6",
        7 Eea7 "EEA7",
    }
}
