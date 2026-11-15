gtpv2c_layout! {
    /// STN-SR (§TS 29.280): its octets, which the crate does not split.
    pub struct StnSr "TS 29.280" { fields { octets "STN-SR": Rest } }
    /// SRVCC Cause (§TS 29.280): its octets, which the crate does not split.
    pub struct SrvccCause "TS 29.280" { fields { octets "SRVCC Cause": Rest } }
    /// IP Address (§8.9): its octets, which the crate does not split.
    pub struct IpAddress "8.9" { fields { octets "IP Address": Rest } }
    /// Indication (§8.12): its octets, which the crate does not split.
    pub struct Indication "8.12" { fields { octets "Indication": Rest } }
    /// Protocol Configuration Options (PCO) (§8.13): its octets, which the crate does not split.
    pub struct Pco "8.13" { fields { octets "Protocol Configuration Options (PCO)": Rest } }
    /// PDN Address Allocation (PAA) (§8.14): its octets, which the crate does not split.
    pub struct Paa "8.14" { fields { octets "PDN Address Allocation (PAA)": Rest } }
    /// Flow Quality of Service (Flow QoS) (§8.16): its octets, which the crate does not split.
    pub struct FlowQualityOfService "8.16" { fields { octets "Flow Quality of Service (Flow QoS)": Rest } }
    /// EPS Bearer Level Traffic Flow Template (Bearer TFT) (§8.19): its octets, which the crate does not split.
    pub struct EpsBearerLevelTrafficFlowTemplate "8.19" { fields { octets "EPS Bearer Level Traffic Flow Template (Bearer TFT)": Rest } }
    /// Traffic Aggregation Description (TAD) (§8.20): its octets, which the crate does not split.
    pub struct Tad "8.20" { fields { octets "Traffic Aggregation Description (TAD)": Rest } }
    /// User Location Information (ULI) (§8.21): its octets, which the crate does not split.
    pub struct Uli "8.21" { fields { octets "User Location Information (ULI)": Rest } }
    /// TMSI (§8.23): its octets, which the crate does not split.
    pub struct Tmsi "8.23" { fields { octets "TMSI": Rest } }
    /// Global CN-Id (§8.24): its octets, which the crate does not split.
    pub struct GlobalCnId "8.24" { fields { octets "Global CN-Id": Rest } }
    /// S103 PDN Data Forwarding Info (S103PDF) (§8.25): its octets, which the crate does not split.
    pub struct S103pdf "8.25" { fields { octets "S103 PDN Data Forwarding Info (S103PDF)": Rest } }
    /// S1-U Data Forwarding Info (S1UDF) (§8.26): its octets, which the crate does not split.
    pub struct S1udf "8.26" { fields { octets "S1-U Data Forwarding Info (S1UDF)": Rest } }
    /// Trace Information (§8.31): its octets, which the crate does not split.
    pub struct TraceInformation "8.31" { fields { octets "Trace Information": Rest } }
    /// Bearer Flags (§8.32): its octets, which the crate does not split.
    pub struct BearerFlags "8.32" { fields { octets "Bearer Flags": Rest } }
    /// Procedure Transaction ID (§8.35): its octets, which the crate does not split.
    pub struct ProcedureTransactionId "8.35" { fields { octets "Procedure Transaction ID": Rest } }
    /// MM Context (GSM Key and Triplets) (§8.38): its octets, which the crate does not split.
    pub struct MmContextGsmKeyAndTriplets "8.38" { fields { octets "MM Context (GSM Key and Triplets)": SecretRest } }
    /// MM Context (UMTS Key, Used Cipher and Quintuplets) (§8.38): its octets, which the crate does not split.
    pub struct MmContextUmtsKeyUsedCipherAndQuintuplets "8.38" { fields { octets "MM Context (UMTS Key, Used Cipher and Quintuplets)": SecretRest } }
    /// MM Context (GSM Key, Used Cipher and Quintuplets) (§8.38): its octets, which the crate does not split.
    pub struct MmContextGsmKeyUsedCipherAndQuintuplets "8.38" { fields { octets "MM Context (GSM Key, Used Cipher and Quintuplets)": SecretRest } }
    /// MM Context (UMTS Key and Quintuplets) (§8.38): its octets, which the crate does not split.
    pub struct MmContextUmtsKeyAndQuintuplets "8.38" { fields { octets "MM Context (UMTS Key and Quintuplets)": SecretRest } }
    /// MM Context (EPS Security Context, Quadruplets and Quintuplets) (§8.38): its octets, which the crate does not split.
    pub struct MmContextEpsSecurityContextQuadrupletsAndQuintuplets "8.38" { fields { octets "MM Context (EPS Security Context, Quadruplets and Quintuplets)": SecretRest } }
    /// MM Context (UMTS Key, Quadruplets and Quintuplets) (§8.38): its octets, which the crate does not split.
    pub struct MmContextUmtsKeyQuadrupletsAndQuintuplets "8.38" { fields { octets "MM Context (UMTS Key, Quadruplets and Quintuplets)": SecretRest } }
    /// PDU Numbers (§8.40): its octets, which the crate does not split.
    pub struct PduNumbers "8.40" { fields { octets "PDU Numbers": Rest } }
    /// P-TMSI (§8.41): its octets, which the crate does not split.
    pub struct PTmsi "8.41" { fields { octets "P-TMSI": Rest } }
    /// P-TMSI Signature (§8.42): its octets, which the crate does not split.
    pub struct PTmsiSignature "8.42" { fields { octets "P-TMSI Signature": Rest } }
    /// Hop Counter (§8.43): its octets, which the crate does not split.
    pub struct HopCounter "8.43" { fields { octets "Hop Counter": Rest } }
    /// UE Time Zone (§8.44): its octets, which the crate does not split.
    pub struct UeTimeZone "8.44" { fields { octets "UE Time Zone": Rest } }
    /// Trace Reference (§8.45): its octets, which the crate does not split.
    pub struct TraceReference "8.45" { fields { octets "Trace Reference": Rest } }
    /// Complete Request Message (§8.46): its octets, which the crate does not split.
    pub struct CompleteRequestMessage "8.46" { fields { octets "Complete Request Message": Rest } }
    /// GUTI (§8.47): its octets, which the crate does not split.
    pub struct Guti "8.47" { fields { octets "GUTI": Rest } }
    /// F-Container (§8.48): its octets, which the crate does not split.
    pub struct FContainer "8.48" { fields { octets "F-Container": Rest } }
    /// F-Cause (§8.49): its octets, which the crate does not split.
    pub struct FCause "8.49" { fields { octets "F-Cause": Rest } }
    /// PLMN ID (§8.50): its octets, which the crate does not split.
    pub struct PlmnId "8.50" { fields { octets "PLMN ID": Rest } }
    /// Target Identification (§8.51): its octets, which the crate does not split.
    pub struct TargetIdentification "8.51" { fields { octets "Target Identification": Rest } }
    /// Packet Flow ID (§8.53): its octets, which the crate does not split.
    pub struct PacketFlowId "8.53" { fields { octets "Packet Flow ID": Rest } }
    /// RAB Context (§8.54): its octets, which the crate does not split.
    pub struct RabContext "8.54" { fields { octets "RAB Context": Rest } }
    /// Source RNC PDCP Context Info (§8.55): its octets, which the crate does not split.
    pub struct SourceRncPdcpContextInfo "8.55" { fields { octets "Source RNC PDCP Context Info": Rest } }
    /// Source Identification (§8.59): its octets, which the crate does not split.
    pub struct SourceIdentification "8.59" { fields { octets "Source Identification": Rest } }
    /// Change Reporting Action (§8.61): its octets, which the crate does not split.
    pub struct ChangeReportingAction "8.61" { fields { octets "Change Reporting Action": Rest } }
    /// Fully Qualified PDN Connection Set Identifier (FQ-CSID) (§8.62): its octets, which the crate does not split.
    pub struct FqCsid "8.62" { fields { octets "Fully Qualified PDN Connection Set Identifier (FQ-CSID)": Rest } }
    /// Channel needed (§8.63): its octets, which the crate does not split.
    pub struct ChannelNeeded "8.63" { fields { octets "Channel needed": Rest } }
    /// eMLPP Priority (§8.64): its octets, which the crate does not split.
    pub struct EmlppPriority "8.64" { fields { octets "eMLPP Priority": Rest } }
    /// Transaction Identifier (TI) (§8.68): its octets, which the crate does not split.
    pub struct Ti "8.68" { fields { octets "Transaction Identifier (TI)": Rest } }
    /// MBMS Session Duration (§8.69): its octets, which the crate does not split.
    pub struct MbmsSessionDuration "8.69" { fields { octets "MBMS Session Duration": Rest } }
    /// MBMS Service Area (§8.70): its octets, which the crate does not split.
    pub struct MbmsServiceArea "8.70" { fields { octets "MBMS Service Area": Rest } }
    /// MBMS Session Identifier (§8.71): its octets, which the crate does not split.
    pub struct MbmsSessionIdentifier "8.71" { fields { octets "MBMS Session Identifier": Rest } }
    /// MBMS Flow Identifier (§8.72): its octets, which the crate does not split.
    pub struct MbmsFlowIdentifier "8.72" { fields { octets "MBMS Flow Identifier": Rest } }
    /// MBMS IP Multicast Distribution (§8.73): its octets, which the crate does not split.
    pub struct MbmsIpMulticastDistribution "8.73" { fields { octets "MBMS IP Multicast Distribution": Rest } }
    /// MBMS Distribution Acknowledge (§8.74): its octets, which the crate does not split.
    pub struct MbmsDistributionAcknowledge "8.74" { fields { octets "MBMS Distribution Acknowledge": Rest } }
    /// RFSP Index (§8.77): its octets, which the crate does not split.
    pub struct RfspIndex "8.77" { fields { octets "RFSP Index": Rest } }
    /// User CSG Information (UCI) (§8.75): its octets, which the crate does not split.
    pub struct Uci "8.75" { fields { octets "User CSG Information (UCI)": Rest } }
    /// CSG Information Reporting Action (§8.76): its octets, which the crate does not split.
    pub struct CsgInformationReportingAction "8.76" { fields { octets "CSG Information Reporting Action": Rest } }
    /// CSG ID (§8.78): its octets, which the crate does not split.
    pub struct CsgId "8.78" { fields { octets "CSG ID": Rest } }
    /// CSG Membership Indication (CMI) (§8.79): its octets, which the crate does not split.
    pub struct Cmi "8.79" { fields { octets "CSG Membership Indication (CMI)": Rest } }
    /// Service indicator (§8.80): its octets, which the crate does not split.
    pub struct ServiceIndicator "8.80" { fields { octets "Service indicator": Rest } }
    /// Detach Type (§8.81): its octets, which the crate does not split.
    pub struct DetachType "8.81" { fields { octets "Detach Type": Rest } }
    /// Local Distinguished Name (LDN) (§8.82): its octets, which the crate does not split.
    pub struct Ldn "8.82" { fields { octets "Local Distinguished Name (LDN)": Rest } }
    /// Node Features (§8.83): its octets, which the crate does not split.
    pub struct NodeFeatures "8.83" { fields { octets "Node Features": Rest } }
    /// MBMS Time to Data Transfer (§8.84): its octets, which the crate does not split.
    pub struct MbmsTimeToDataTransfer "8.84" { fields { octets "MBMS Time to Data Transfer": Rest } }
    /// Throttling (§8.85): its octets, which the crate does not split.
    pub struct Throttling "8.85" { fields { octets "Throttling": Rest } }
    /// Allocation/Retention Priority (ARP) (§8.86): its octets, which the crate does not split.
    pub struct Arp "8.86" { fields { octets "Allocation/Retention Priority (ARP)": Rest } }
    /// EPC Timer (§8.87): its octets, which the crate does not split.
    pub struct EpcTimer "8.87" { fields { octets "EPC Timer": Rest } }
    /// Signalling Priority Indication (§8.88): its octets, which the crate does not split.
    pub struct SignallingPriorityIndication "8.88" { fields { octets "Signalling Priority Indication": Rest } }
    /// Temporary Mobile Group Identity (TMGI) (§8.89): its octets, which the crate does not split.
    pub struct Tmgi "8.89" { fields { octets "Temporary Mobile Group Identity (TMGI)": Rest } }
    /// Additional MM context for SRVCC (§8.90): its octets, which the crate does not split.
    pub struct AdditionalMmContextForSrvcc "8.90" { fields { octets "Additional MM context for SRVCC": Rest } }
    /// Additional flags for SRVCC (§8.91): its octets, which the crate does not split.
    pub struct AdditionalFlagsForSrvcc "8.91" { fields { octets "Additional flags for SRVCC": Rest } }
    /// MDT Configuration (§8.93): its octets, which the crate does not split.
    pub struct MdtConfiguration "8.93" { fields { octets "MDT Configuration": Rest } }
    /// Additional Protocol Configuration Options (APCO) (§8.94): its octets, which the crate does not split.
    pub struct Apco "8.94" { fields { octets "Additional Protocol Configuration Options (APCO)": Rest } }
    /// Absolute Time of MBMS Data Transfer (§8.95): its octets, which the crate does not split.
    pub struct AbsoluteTimeOfMbmsDataTransfer "8.95" { fields { octets "Absolute Time of MBMS Data Transfer": Rest } }
    /// H(e)NB Information Reporting (§8.96): its octets, which the crate does not split.
    pub struct HENbInformationReporting "8.96" { fields { octets "H(e)NB Information Reporting": Rest } }
    /// IPv4 Configuration Parameters (IP4CP) (§8.97): its octets, which the crate does not split.
    pub struct Ip4cp "8.97" { fields { octets "IPv4 Configuration Parameters (IP4CP)": Rest } }
    /// Change to Report Flags (§8.98): its octets, which the crate does not split.
    pub struct ChangeToReportFlags "8.98" { fields { octets "Change to Report Flags": Rest } }
    /// Action Indication (§8.99): its octets, which the crate does not split.
    pub struct ActionIndication "8.99" { fields { octets "Action Indication": Rest } }
    /// TWAN Identifier (§8.100): its octets, which the crate does not split.
    pub struct TwanIdentifier "8.100" { fields { octets "TWAN Identifier": Rest } }
    /// ULI Timestamp (§8.101): its octets, which the crate does not split.
    pub struct UliTimestamp "8.101" { fields { octets "ULI Timestamp": Rest } }
    /// MBMS Flags (§8.102): its octets, which the crate does not split.
    pub struct MbmsFlags "8.102" { fields { octets "MBMS Flags": Rest } }
    /// RAN/NAS Cause (§8.103): its octets, which the crate does not split.
    pub struct RanNasCause "8.103" { fields { octets "RAN/NAS Cause": Rest } }
    /// CN Operator Selection Entity (§8.104): its octets, which the crate does not split.
    pub struct CnOperatorSelectionEntity "8.104" { fields { octets "CN Operator Selection Entity": Rest } }
    /// Trusted WLAN Mode Indication (§8.105): its octets, which the crate does not split.
    pub struct TrustedWlanModeIndication "8.105" { fields { octets "Trusted WLAN Mode Indication": Rest } }
    /// Node Number (§8.106): its octets, which the crate does not split.
    pub struct NodeNumber "8.106" { fields { octets "Node Number": Rest } }
    /// Node Identifier (§8.107): its octets, which the crate does not split.
    pub struct NodeIdentifier "8.107" { fields { octets "Node Identifier": Rest } }
    /// Presence Reporting Area Action (§8.108): its octets, which the crate does not split.
    pub struct PresenceReportingAreaAction "8.108" { fields { octets "Presence Reporting Area Action": Rest } }
    /// Presence Reporting Area Information (§8.109): its octets, which the crate does not split.
    pub struct PresenceReportingAreaInformation "8.109" { fields { octets "Presence Reporting Area Information": Rest } }
    /// TWAN Identifier Timestamp (§8.110): its octets, which the crate does not split.
    pub struct TwanIdentifierTimestamp "8.110" { fields { octets "TWAN Identifier Timestamp": Rest } }
    /// Metric (§8.113): its octets, which the crate does not split.
    pub struct Metric "8.113" { fields { octets "Metric": Rest } }
    /// Sequence Number (§8.114): its octets, which the crate does not split.
    pub struct SequenceNumber "8.114" { fields { octets "Sequence Number": Rest } }
    /// APN and Relative Capacity (§8.115): its octets, which the crate does not split.
    pub struct ApnAndRelativeCapacity "8.115" { fields { octets "APN and Relative Capacity": Rest } }
    /// WLAN Offloadability Indication (§8.116): its octets, which the crate does not split.
    pub struct WlanOffloadabilityIndication "8.116" { fields { octets "WLAN Offloadability Indication": Rest } }
    /// Paging and Service Information (§8.117): its octets, which the crate does not split.
    pub struct PagingAndServiceInformation "8.117" { fields { octets "Paging and Service Information": Rest } }
    /// Integer Number (§8.118): its octets, which the crate does not split.
    pub struct IntegerNumber "8.118" { fields { octets "Integer Number": Rest } }
    /// Millisecond Time Stamp (§8.119): its octets, which the crate does not split.
    pub struct MillisecondTimeStamp "8.119" { fields { octets "Millisecond Time Stamp": Rest } }
    /// Monitoring Event Information (§8.120): its octets, which the crate does not split.
    pub struct MonitoringEventInformation "8.120" { fields { octets "Monitoring Event Information": Rest } }
    /// ECGI List (§8.121): its octets, which the crate does not split.
    pub struct EcgiList "8.121" { fields { octets "ECGI List": Rest } }
    /// Remote User ID (§8.123): its octets, which the crate does not split.
    pub struct RemoteUserId "8.123" { fields { octets "Remote User ID": Rest } }
    /// Remote UE IP information (§8.124): its octets, which the crate does not split.
    pub struct RemoteUeIpInformation "8.124" { fields { octets "Remote UE IP information": Rest } }
    /// CIoT Optimizations Support Indication (§8.125): its octets, which the crate does not split.
    pub struct CiotOptimizationsSupportIndication "8.125" { fields { octets "CIoT Optimizations Support Indication": Rest } }
    /// Header Compression Configuration (§8.127): its octets, which the crate does not split.
    pub struct HeaderCompressionConfiguration "8.127" { fields { octets "Header Compression Configuration": Rest } }
    /// Extended Protocol Configuration Options (ePCO) (§8.128): its octets, which the crate does not split.
    pub struct ExtendedProtocolConfigurationOptions "8.128" { fields { octets "Extended Protocol Configuration Options (ePCO)": Rest } }
    /// Serving PLMN Rate Control (§8.129): its octets, which the crate does not split.
    pub struct ServingPlmnRateControl "8.129" { fields { octets "Serving PLMN Rate Control": Rest } }
    /// Counter (§8.130): its octets, which the crate does not split.
    pub struct Counter "8.130" { fields { octets "Counter": Rest } }
    /// Mapped UE Usage Type (§8.131): its octets, which the crate does not split.
    pub struct MappedUeUsageType "8.131" { fields { octets "Mapped UE Usage Type": Rest } }
    /// Secondary RAT Usage Data Report (§8.132): its octets, which the crate does not split.
    pub struct SecondaryRatUsageDataReport "8.132" { fields { octets "Secondary RAT Usage Data Report": Rest } }
    /// UP Function Selection Indication Flags (§8.133): its octets, which the crate does not split.
    pub struct UpFunctionSelectionIndicationFlags "8.133" { fields { octets "UP Function Selection Indication Flags": Rest } }
    /// Maximum Packet Loss Rate (§8.134): its octets, which the crate does not split.
    pub struct MaximumPacketLossRate "8.134" { fields { octets "Maximum Packet Loss Rate": Rest } }
    /// APN Rate Control Status (§8.135): its octets, which the crate does not split.
    pub struct ApnRateControlStatus "8.135" { fields { octets "APN Rate Control Status": Rest } }
    /// Extended Trace Information (§8.136): its octets, which the crate does not split.
    pub struct ExtendedTraceInformation "8.136" { fields { octets "Extended Trace Information": Rest } }
    /// Monitoring Event Extension Information (§8.137): its octets, which the crate does not split.
    pub struct MonitoringEventExtensionInformation "8.137" { fields { octets "Monitoring Event Extension Information": Rest } }
    /// Additional RRM Policy Index (§8.138): its octets, which the crate does not split.
    pub struct AdditionalRrmPolicyIndex "8.138" { fields { octets "Additional RRM Policy Index": Rest } }
    /// Services Authorized (§8.141): its octets, which the crate does not split.
    pub struct ServicesAuthorized "8.141" { fields { octets "Services Authorized": Rest } }
    /// Bit Rate (§8.142): its octets, which the crate does not split.
    pub struct BitRate "8.142" { fields { octets "Bit Rate": Rest } }
    /// PC5 QoS Flow (§8.143): its octets, which the crate does not split.
    pub struct Pc5QosFlow "8.143" { fields { octets "PC5 QoS Flow": Rest } }
    /// SGi PtP Tunnel Address (§8.144): its octets, which the crate does not split.
    pub struct SgiPtpTunnelAddress "8.144" { fields { octets "SGi PtP Tunnel Address": Rest } }
    /// PGW FQDN (§8.146): its octets, which the crate does not split.
    pub struct PgwFqdn "8.146" { fields { octets "PGW FQDN": Rest } }
    /// Group Id (§8.147): its octets, which the crate does not split.
    pub struct GroupId "8.147" { fields { octets "Group Id": Rest } }
    /// PSCell ID (§8.148): its octets, which the crate does not split.
    pub struct PscellId "8.148" { fields { octets "PSCell ID": Rest } }
    /// UP Security Policy (§8.149): its octets, which the crate does not split.
    pub struct UpSecurityPolicy "8.149" { fields { octets "UP Security Policy": Rest } }
    /// Alternative IMSI (§8.150): its octets, which the crate does not split.
    pub struct AlternativeImsi "8.150" { fields { octets "Alternative IMSI": Rest } }
    /// NF Instance ID (§8.151): its octets, which the crate does not split.
    pub struct NfInstanceId "8.151" { fields { octets "NF Instance ID": Rest } }
    /// Timer in Seconds (§8.152): its octets, which the crate does not split.
    pub struct TimerInSeconds "8.152" { fields { octets "Timer in Seconds": Rest } }
    /// MDT Configuration NR (§8.153): its octets, which the crate does not split.
    pub struct MdtConfigurationNr "8.153" { fields { octets "MDT Configuration NR": Rest } }
    /// URI (§8.154): its octets, which the crate does not split.
    pub struct Uri "8.154" { fields { octets "URI": Rest } }
    /// Private Extension (§8.67): its octets, which the crate does not split.
    pub struct PrivateExtension "8.67" { fields { octets "Private Extension": Rest } }
}

listed_value!(
    Imsi,
    Cause,
    Recovery,
    StnSr,
    SrvccCause,
    Apn,
    Ambr,
    Ebi,
    IpAddress,
    Mei,
    Msisdn,
    Indication,
    Pco,
    Paa,
    BearerLevelQualityOfService,
    FlowQualityOfService,
    RatType,
    ServingNetwork,
    EpsBearerLevelTrafficFlowTemplate,
    Tad,
    Uli,
    FTeid,
    Tmsi,
    GlobalCnId,
    S103pdf,
    S1udf,
    DelayValue,
    ChargingId,
    ChargingCharacteristics,
    TraceInformation,
    BearerFlags,
    PdnType,
    ProcedureTransactionId,
    MmContextGsmKeyAndTriplets,
    MmContextUmtsKeyUsedCipherAndQuintuplets,
    MmContextGsmKeyUsedCipherAndQuintuplets,
    MmContextUmtsKeyAndQuintuplets,
    MmContextEpsSecurityContextQuadrupletsAndQuintuplets,
    MmContextUmtsKeyQuadrupletsAndQuintuplets,
    PduNumbers,
    PTmsi,
    PTmsiSignature,
    HopCounter,
    UeTimeZone,
    TraceReference,
    CompleteRequestMessage,
    Guti,
    FContainer,
    FCause,
    PlmnId,
    TargetIdentification,
    PacketFlowId,
    RabContext,
    SourceRncPdcpContextInfo,
    PortNumber,
    ApnRestriction,
    SelectionMode,
    SourceIdentification,
    ChangeReportingAction,
    FqCsid,
    ChannelNeeded,
    EmlppPriority,
    NodeType,
    Fqdn,
    Ti,
    MbmsSessionDuration,
    MbmsServiceArea,
    MbmsSessionIdentifier,
    MbmsFlowIdentifier,
    MbmsIpMulticastDistribution,
    MbmsDistributionAcknowledge,
    RfspIndex,
    Uci,
    CsgInformationReportingAction,
    CsgId,
    Cmi,
    ServiceIndicator,
    DetachType,
    Ldn,
    NodeFeatures,
    MbmsTimeToDataTransfer,
    Throttling,
    Arp,
    EpcTimer,
    SignallingPriorityIndication,
    Tmgi,
    AdditionalMmContextForSrvcc,
    AdditionalFlagsForSrvcc,
    MdtConfiguration,
    Apco,
    AbsoluteTimeOfMbmsDataTransfer,
    HENbInformationReporting,
    Ip4cp,
    ChangeToReportFlags,
    ActionIndication,
    TwanIdentifier,
    UliTimestamp,
    MbmsFlags,
    RanNasCause,
    CnOperatorSelectionEntity,
    TrustedWlanModeIndication,
    NodeNumber,
    NodeIdentifier,
    PresenceReportingAreaAction,
    PresenceReportingAreaInformation,
    TwanIdentifierTimestamp,
    Metric,
    SequenceNumber,
    ApnAndRelativeCapacity,
    WlanOffloadabilityIndication,
    PagingAndServiceInformation,
    IntegerNumber,
    MillisecondTimeStamp,
    MonitoringEventInformation,
    EcgiList,
    RemoteUserId,
    RemoteUeIpInformation,
    CiotOptimizationsSupportIndication,
    HeaderCompressionConfiguration,
    ExtendedProtocolConfigurationOptions,
    ServingPlmnRateControl,
    Counter,
    MappedUeUsageType,
    SecondaryRatUsageDataReport,
    UpFunctionSelectionIndicationFlags,
    MaximumPacketLossRate,
    ApnRateControlStatus,
    ExtendedTraceInformation,
    MonitoringEventExtensionInformation,
    AdditionalRrmPolicyIndex,
    ServicesAuthorized,
    BitRate,
    Pc5QosFlow,
    SgiPtpTunnelAddress,
    PgwFqdn,
    GroupId,
    PscellId,
    UpSecurityPolicy,
    AlternativeImsi,
    NfInstanceId,
    TimerInSeconds,
    MdtConfigurationNr,
    Uri,
    PrivateExtension,
);
