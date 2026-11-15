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
