gtpv2c_ies! {
    1 "International Mobile Subscriber Identity (IMSI)" variable "8.3" => Imsi;
    2 "Cause" variable "8.4" => Cause;
    3 "Recovery (Restart Counter)" variable "8.5" => Recovery;
    51 "STN-SR" variable "TS 29.280" => StnSr;
    56 "SRVCC Cause" variable "TS 29.280" => SrvccCause;
    71 "Access Point Name (APN)" variable "8.6" => Apn;
    72 "Aggregate Maximum Bit Rate (AMBR)" fixed(8) "8.7" => Ambr;
    73 "EPS Bearer ID (EBI)" extendable(1) "8.8" => Ebi;
    74 "IP Address" variable "8.9" => IpAddress;
    75 "Mobile Equipment Identity (MEI)" variable "8.10" => Mei;
    76 "MSISDN" variable "8.11" => Msisdn;
    77 "Indication" extendable(2) "8.12" => Indication;
    78 "Protocol Configuration Options (PCO)" variable "8.13" => Pco;
    79 "PDN Address Allocation (PAA)" variable "8.14" => Paa;
    80 "Bearer Level Quality of Service (Bearer QoS)" extendable(22) "8.15" => BearerLevelQualityOfService;
    81 "Flow Quality of Service (Flow QoS)" extendable(21) "8.16" => FlowQualityOfService;
    82 "RAT Type" extendable(1) "8.17" => RatType;
    83 "Serving Network" extendable(3) "8.18" => ServingNetwork;
    84 "EPS Bearer Level Traffic Flow Template (Bearer TFT)" variable "8.19" => EpsBearerLevelTrafficFlowTemplate;
    85 "Traffic Aggregation Description (TAD)" variable "8.20" => Tad;
    86 "User Location Information (ULI)" extendable "8.21" => Uli;
    87 "Fully Qualified Tunnel Endpoint Identifier (F-TEID)" extendable "8.22" => FTeid;
    88 "TMSI" variable "8.23" => Tmsi;
    89 "Global CN-Id" variable "8.24" => GlobalCnId;
    90 "S103 PDN Data Forwarding Info (S103PDF)" variable "8.25" => S103pdf;
    91 "S1-U Data Forwarding Info (S1UDF)" variable "8.26" => S1udf;
    92 "Delay Value" extendable(1) "8.27" => DelayValue;
    93 "Bearer Context" grouped "8.28" => BearerContextToBeCreatedInCreateSessionRequest, BearerContextToBeRemovedInCreateSessionRequest, BearerContextCreatedInCreateSessionResponse, BearerContextMarkedForRemovalInACreateSessionResponse, BearerContextInCreateBearerRequest, BearerContextInCreateBearerResponse, BearerContextToBeModifiedInModifyBearerRequest, BearerContextToBeRemovedInModifyBearerRequest, BearerContextModifiedInModifyBearerResponse, BearerContextMarkedForRemovalInModifyBearerResponse, BearerContextInDeleteBearerRequest, BearerContextInDeleteBearerResponse, BearerContextInModifyBearerCommand, BearerContextInUpdateBearerRequest, BearerContextInUpdateBearerResponse, BearerContextInDeleteBearerCommand, BearerContextInDeleteBearerFailureIndication, BearerContextInCreateIndirectDataForwardingTunnelRequest, BearerContextInCreateIndirectDataForwardingTunnelResponse, BearerContextToBeModifiedInModifyAccessBearersRequest, BearerContextToBeRemovedInModifyAccessBearersRequest, BearerContextModifiedInModifyAccessBearersResponse, BearerContextMarkedForRemovalInModifyAccessBearersResponse, BearerContextInMmeSgsnAmfUeEpsPdnConnectionsInForwardRelocationRequest, BearerContextT7_3_2_2, BearerContextInMmeSgsnAmfUeEpsPdnConnectionsInContextResponse, BearerContextInContextAcknowledge;
    94 "Charging ID" extendable(4) "8.29" => ChargingId;
    95 "Charging Characteristics" extendable(2) "8.30" => ChargingCharacteristics;
    96 "Trace Information" variable "8.31" => TraceInformation;
    97 "Bearer Flags" extendable(1) "8.32" => BearerFlags;
    99 "PDN Type" extendable(1) "8.34" => PdnType;
    100 "Procedure Transaction ID" extendable(1) "8.35" => ProcedureTransactionId;
    103 "MM Context (GSM Key and Triplets)" extendable "8.38" => MmContextGsmKeyAndTriplets;
    104 "MM Context (UMTS Key, Used Cipher and Quintuplets)" extendable "8.38" => MmContextUmtsKeyUsedCipherAndQuintuplets;
    105 "MM Context (GSM Key, Used Cipher and Quintuplets)" extendable "8.38" => MmContextGsmKeyUsedCipherAndQuintuplets;
    106 "MM Context (UMTS Key and Quintuplets)" extendable "8.38" => MmContextUmtsKeyAndQuintuplets;
    107 "MM Context (EPS Security Context, Quadruplets and Quintuplets)" extendable "8.38" => MmContextEpsSecurityContextQuadrupletsAndQuintuplets;
    108 "MM Context (UMTS Key, Quadruplets and Quintuplets)" extendable "8.38" => MmContextUmtsKeyQuadrupletsAndQuintuplets;
    109 "PDN Connection" grouped "8.39" => MmeSgsnAmfUeEpsPdnConnectionsInForwardRelocationRequest, MmeSgsnAmfUeEpsPdnConnectionsInContextResponse;
    110 "PDU Numbers" extendable(9) "8.40" => PduNumbers;
    111 "P-TMSI" variable "8.41" => PTmsi;
    112 "P-TMSI Signature" variable "8.42" => PTmsiSignature;
    113 "Hop Counter" extendable(1) "8.43" => HopCounter;
    114 "UE Time Zone" extendable(2) "8.44" => UeTimeZone;
    115 "Trace Reference" fixed(6) "8.45" => TraceReference;
    116 "Complete Request Message" variable "8.46" => CompleteRequestMessage;
    117 "GUTI" variable "8.47" => Guti;
    118 "F-Container" variable "8.48" => FContainer;
    119 "F-Cause" variable "8.49" => FCause;
    120 "PLMN ID" variable "8.50" => PlmnId;
    121 "Target Identification" variable "8.51" => TargetIdentification;
    123 "Packet Flow ID" variable "8.53" => PacketFlowId;
    124 "RAB Context" fixed(9) "8.54" => RabContext;
    125 "Source RNC PDCP Context Info" variable "8.55" => SourceRncPdcpContextInfo;
    126 "Port Number" extendable(2) "8.56" => PortNumber;
    127 "APN Restriction" extendable(1) "8.57" => ApnRestriction;
    128 "Selection Mode" extendable(1) "8.58" => SelectionMode;
    129 "Source Identification" variable "8.59" => SourceIdentification;
    131 "Change Reporting Action" variable "8.61" => ChangeReportingAction;
    132 "Fully Qualified PDN Connection Set Identifier (FQ-CSID)" extendable "8.62" => FqCsid;
    133 "Channel needed" variable "8.63" => ChannelNeeded;
    134 "eMLPP Priority" variable "8.64" => EmlppPriority;
    135 "Node Type" extendable(1) "8.65" => NodeType;
    136 "Fully Qualified Domain Name (FQDN)" variable "8.66" => Fqdn;
    137 "Transaction Identifier (TI)" variable "8.68" => Ti;
    138 "MBMS Session Duration" extendable(3) "8.69" => MbmsSessionDuration;
    139 "MBMS Service Area" variable "8.70" => MbmsServiceArea;
    140 "MBMS Session Identifier" extendable(1) "8.71" => MbmsSessionIdentifier;
    141 "MBMS Flow Identifier" extendable(2) "8.72" => MbmsFlowIdentifier;
    142 "MBMS IP Multicast Distribution" extendable "8.73" => MbmsIpMulticastDistribution;
    143 "MBMS Distribution Acknowledge" extendable(1) "8.74" => MbmsDistributionAcknowledge;
    144 "RFSP Index" fixed(2) "8.77" => RfspIndex;
    145 "User CSG Information (UCI)" extendable(8) "8.75" => Uci;
    146 "CSG Information Reporting Action" extendable(1) "8.76" => CsgInformationReportingAction;
    147 "CSG ID" extendable(4) "8.78" => CsgId;
    148 "CSG Membership Indication (CMI)" extendable(1) "8.79" => Cmi;
    149 "Service indicator" fixed(1) "8.80" => ServiceIndicator;
    150 "Detach Type" fixed(1) "8.81" => DetachType;
    151 "Local Distinguished Name (LDN)" variable "8.82" => Ldn;
    152 "Node Features" extendable(1) "8.83" => NodeFeatures;
    153 "MBMS Time to Data Transfer" extendable(1) "8.84" => MbmsTimeToDataTransfer;
    154 "Throttling" extendable(2) "8.85" => Throttling;
    155 "Allocation/Retention Priority (ARP)" extendable(1) "8.86" => Arp;
    156 "EPC Timer" extendable(1) "8.87" => EpcTimer;
    157 "Signalling Priority Indication" extendable(1) "8.88" => SignallingPriorityIndication;
    158 "Temporary Mobile Group Identity (TMGI)" extendable(6) "8.89" => Tmgi;
    159 "Additional MM context for SRVCC" extendable "8.90" => AdditionalMmContextForSrvcc;
    160 "Additional flags for SRVCC" extendable(1) "8.91" => AdditionalFlagsForSrvcc;
    162 "MDT Configuration" extendable "8.93" => MdtConfiguration;
    163 "Additional Protocol Configuration Options (APCO)" extendable "8.94" => Apco;
    164 "Absolute Time of MBMS Data Transfer" extendable(8) "8.95" => AbsoluteTimeOfMbmsDataTransfer;
    165 "H(e)NB Information Reporting" extendable(1) "8.96" => HENbInformationReporting;
    166 "IPv4 Configuration Parameters (IP4CP)" extendable(5) "8.97" => Ip4cp;
    167 "Change to Report Flags" extendable(1) "8.98" => ChangeToReportFlags;
    168 "Action Indication" extendable(1) "8.99" => ActionIndication;
    169 "TWAN Identifier" extendable "8.100" => TwanIdentifier;
    170 "ULI Timestamp" extendable(4) "8.101" => UliTimestamp;
    171 "MBMS Flags" extendable(1) "8.102" => MbmsFlags;
    172 "RAN/NAS Cause" extendable "8.103" => RanNasCause;
    173 "CN Operator Selection Entity" extendable(1) "8.104" => CnOperatorSelectionEntity;
    174 "Trusted WLAN Mode Indication" extendable(1) "8.105" => TrustedWlanModeIndication;
    175 "Node Number" extendable "8.106" => NodeNumber;
    176 "Node Identifier" extendable "8.107" => NodeIdentifier;
    177 "Presence Reporting Area Action" extendable "8.108" => PresenceReportingAreaAction;
    178 "Presence Reporting Area Information" extendable(4) "8.109" => PresenceReportingAreaInformation;
    179 "TWAN Identifier Timestamp" extendable(4) "8.110" => TwanIdentifierTimestamp;
    180 "Overload Control Information" grouped "8.111" => OverloadControlInformationInCreateSessionRequest, OverloadControlInformationInCreateSessionResponse, OverloadControlInformationInCreateBearerRequest, OverloadControlInformationInCreateBearerResponse, OverloadControlInformationInBearerResourceCommand, OverloadControlInformationInBearerResourceFailureIndication, OverloadControlInformationInModifyBearerRequest, OverloadControlInformationInModifyBearerResponse, OverloadControlInformationInDeleteSessionRequest, OverloadControlInformationInDeleteBearerRequest, OverloadControlInformationInDeleteSessionResponse, OverloadControlInformationInDeleteBearerResponse, OverloadControlInformationInDownlinkDataNotification, OverloadControlInformationInModifyBearerCommand, OverloadControlInformationInModifyBearerFailureIndication, OverloadControlInformationInUpdateBearerRequest, OverloadControlInformationInUpdateBearerResponse, OverloadControlInformationInDeleteBearerCommand, OverloadControlInformationInDeleteBearerFailureIndication, OverloadControlInformationInReleaseAccessBearersResponse, OverloadControlInformationInModifyAccessBearersResponse;
    181 "Load Control Information" grouped "8.112" => LoadControlInformationInCreateSessionResponse, LoadControlInformationInCreateBearerRequest, LoadControlInformationInModifyBearerResponse, LoadControlInformationInDeleteBearerRequest, LoadControlInformationInDeleteSessionResponse, LoadControlInformationInDownlinkDataNotification, LoadControlInformationInUpdateBearerRequest, LoadControlInformationInReleaseAccessBearersResponse, LoadControlInformationInModifyAccessBearersResponse;
    182 "Metric" fixed(1) "8.113" => Metric;
    183 "Sequence Number" fixed(4) "8.114" => SequenceNumber;
    184 "APN and Relative Capacity" extendable "8.115" => ApnAndRelativeCapacity;
    185 "WLAN Offloadability Indication" extendable(1) "8.116" => WlanOffloadabilityIndication;
    186 "Paging and Service Information" extendable "8.117" => PagingAndServiceInformation;
    187 "Integer Number" variable "8.118" => IntegerNumber;
    188 "Millisecond Time Stamp" extendable(6) "8.119" => MillisecondTimeStamp;
    189 "Monitoring Event Information" extendable "8.120" => MonitoringEventInformation;
    190 "ECGI List" extendable "8.121" => EcgiList;
    191 "Remote UE Context" grouped "8.122" => RemoteUeContextConnectedInCreateSessionRequest, RemoteUeContextConnectedInRemoteUeReportNotification, RemoteUeContextDisconnectedInRemoteUeReportNotification, RemoteUeContextConnectedInMmeSgsnUeEpsPdnConnectionsInForwardRelocationRequest, RemoteUeContextConnectedInMmeSgsnUeEpsPdnConnectionsInContextResponse;
    192 "Remote User ID" extendable "8.123" => RemoteUserId;
    193 "Remote UE IP information" variable "8.124" => RemoteUeIpInformation;
    194 "CIoT Optimizations Support Indication" extendable(1) "8.125" => CiotOptimizationsSupportIndication;
    195 "SCEF PDN Connection" grouped "8.126" => MmeUeScefPdnConnectionsInForwardRelocationRequest, MmeSgsnUeScefPdnConnectionsInContextResponse;
    196 "Header Compression Configuration" extendable(4) "8.127" => HeaderCompressionConfiguration;
    197 "Extended Protocol Configuration Options (ePCO)" variable "8.128" => ExtendedProtocolConfigurationOptions;
    198 "Serving PLMN Rate Control" extendable(4) "8.129" => ServingPlmnRateControl;
    199 "Counter" extendable(5) "8.130" => Counter;
    200 "Mapped UE Usage Type" extendable(2) "8.131" => MappedUeUsageType;
    201 "Secondary RAT Usage Data Report" extendable(27) "8.132" => SecondaryRatUsageDataReport;
    202 "UP Function Selection Indication Flags" extendable(1) "8.133" => UpFunctionSelectionIndicationFlags;
    203 "Maximum Packet Loss Rate" extendable(1) "8.134" => MaximumPacketLossRate;
    204 "APN Rate Control Status" extendable(20) "8.135" => ApnRateControlStatus;
    205 "Extended Trace Information" extendable "8.136" => ExtendedTraceInformation;
    206 "Monitoring Event Extension Information" extendable "8.137" => MonitoringEventExtensionInformation;
    207 "Additional RRM Policy Index" fixed(4) "8.138" => AdditionalRrmPolicyIndex;
    208 "V2X Context" grouped "8.139" => SubscribedV2xInformationInForwardRelocationRequest;
    209 "PC5 QoS Parameters" grouped "8.140" => Pc5QosParametersInForwardRelocationRequest;
    210 "Services Authorized" extendable(2) "8.141" => ServicesAuthorized;
    211 "Bit Rate" extendable(4) "8.142" => BitRate;
    212 "PC5 QoS Flow" extendable(11) "8.143" => Pc5QosFlow;
    213 "SGi PtP Tunnel Address" extendable(1) "8.144" => SgiPtpTunnelAddress;
    214 "PGW Change Info" grouped "8.145" => PgwChangeInfoInCreateSessionResponse, PgwChangeInfoInCreateBearerRequest, PgwChangeInfoInModifyBearerResponse, PgwChangeInfoInDeleteBearerRequest, PgwChangeInfoInUpdateBearerRequest, PgwChangeInfoInForwardRelocationRequest, PgwChangeInfoInContextResponse;
    215 "PGW FQDN" extendable "8.146" => PgwFqdn;
    216 "Group Id" variable "8.147" => GroupId;
    217 "PSCell ID" fixed(8) "8.148" => PscellId;
    218 "UP Security Policy" extendable(1) "8.149" => UpSecurityPolicy;
    219 "Alternative IMSI" variable "8.150" => AlternativeImsi;
    220 "NF Instance ID" fixed(36) "8.151" => NfInstanceId;
    221 "Timer in Seconds" extendable(3) "8.152" => TimerInSeconds;
    222 "MDT Configuration NR" extendable "8.153" => MdtConfigurationNr;
    223 "URI" variable "8.154" => Uri;
    255 "Private Extension" variable "8.67" => PrivateExtension;
}

gtpv2c_table! {
    /// Information Elements in Echo Request (Table 7.1.1-1).
    message EchoRequest = 1, "Echo Request", "7.1.1-1" {
        recovery: "Recovery" M Recovery 0 [],
        sending_node_features: "Sending Node Features" CO NodeFeatures 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in Echo Response (Table 7.1.2-1).
    message EchoResponse = 2, "Echo Response", "7.1.2-1" {
        recovery: "Recovery" M Recovery 0 [],
        sending_node_features: "Sending Node Features" CO NodeFeatures 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Create Session Request (Table 7.2.1-1).
    message CreateSessionRequest = 32, "Create Session Request", "7.2.1-1" {
        imsi: "IMSI" C Imsi 0 [],
        msisdn: "MSISDN" CO Msisdn 0 [],
        me_identity: "ME Identity (MEI)" CO Mei 0 [],
        user_location_information: "User Location Information (ULI)" CO Uli 0 [],
        serving_network: "Serving Network" CO ServingNetwork 0 [],
        rat_type: "RAT Type" M RatType 0 [],
        indication_flags: "Indication Flags" C Indication 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" M FTeid 0 [],
        pgw_s5_s8_address_for_control_plane_or_pmip: "PGW S5/S8 Address for Control Plane or PMIP" C FTeid 1 [],
        access_point_name: "Access Point Name (APN)" M Apn 0 [],
        selection_mode: "Selection Mode" CO SelectionMode 0 [],
        pdn_type: "PDN Type" CO PdnType 0 [],
        pdn_address_allocation: "PDN Address Allocation (PAA)" CO Paa 0 [],
        maximum_apn_restriction: "Maximum APN Restriction" C ApnRestriction 0 [],
        aggregate_maximum_bit_rate: "Aggregate Maximum Bit Rate (APN-AMBR)" C Ambr 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID" C Ebi 0 [],
        trusted_wlan_mode_indication: "Trusted WLAN Mode Indication" CO TrustedWlanModeIndication 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        bearer_contexts_to_be_created: "Bearer Contexts to be created" M* BearerContextToBeCreatedInCreateSessionRequest 0 [],
        bearer_contexts_to_be_removed: "Bearer Contexts to be removed" C* BearerContextToBeRemovedInCreateSessionRequest 1 [],
        trace_information: "Trace Information" C TraceInformation 0 [],
        recovery: "Recovery" C Recovery 0 [],
        mme_fq_csid: "MME-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        epdg_fq_csid: "ePDG-FQ-CSID" C FqCsid 2 [],
        twan_fq_csid: "TWAN-FQ-CSID" C FqCsid 3 [],
        ue_time_zone: "UE Time Zone" O UeTimeZone 0 [],
        user_csg_information: "User CSG Information (UCI)" CO Uci 0 [],
        charging_characteristics: "Charging Characteristics" C ChargingCharacteristics 0 [],
        mme_s4_sgsn_ldn: "MME/S4-SGSN LDN" O Ldn 0 [],
        sgw_ldn: "SGW LDN" O Ldn 1 [],
        epdg_ldn: "ePDG LDN" O Ldn 2 [],
        twan_ldn: "TWAN LDN" O Ldn 3 [],
        signalling_priority_indication: "Signalling Priority Indication" CO SignallingPriorityIndication 0 [],
        ue_local_ip_address: "UE Local IP Address" O IpAddress 0 [],
        ue_udp_port: "UE UDP Port" O PortNumber 0 [],
        additional_protocol_configuration_options: "Additional Protocol Configuration Options (APCO)" O Apco 0 [],
        hnb_local_ip_address: "H(e)NB Local IP Address" O IpAddress 1 [],
        hnb_udp_port: "H(e)NB UDP Port" O PortNumber 1 [],
        mme_s4_sgsn_identifier: "MME/S4-SGSN Identifier" CO IpAddress 2 [],
        twan_identifier: "TWAN Identifier" CO TwanIdentifier 0 [],
        epdg_ip_address: "ePDG IP Address" O IpAddress 3 [],
        cn_operator_selection_entity: "CN Operator Selection Entity" CO CnOperatorSelectionEntity 0 [],
        presence_reporting_area_information: "Presence Reporting Area Information" CO* PresenceReportingAreaInformation 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInCreateSessionRequest 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInCreateSessionRequest 1 [],
        twan_epdg_s_overload_control_information: "TWAN/ePDG's Overload Control Information" O OverloadControlInformationInCreateSessionRequest 2 [],
        origination_time_stamp: "Origination Time Stamp" CO MillisecondTimeStamp 0 [],
        maximum_wait_time: "Maximum Wait Time" CO IntegerNumber 0 [],
        wlan_location_information: "WLAN Location Information" O TwanIdentifier 1 [],
        wlan_location_timestamp: "WLAN Location Timestamp" O TwanIdentifierTimestamp 0 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        remote_ue_context_connected: "Remote UE Context Connected" CO* RemoteUeContextConnectedInCreateSessionRequest 0 [],
        n3gpp_aaa_server_identifier: "3GPP AAA Server Identifier" O NodeIdentifier 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
        serving_plmn_rate_control: "Serving PLMN Rate Control" CO ServingPlmnRateControl 0 [],
        mo_exception_data_counter: "MO Exception Data Counter" CO Counter 0 [],
        ue_tcp_port: "UE TCP Port" O PortNumber 2 [],
        mapped_ue_usage_type: "Mapped UE Usage Type" CO MappedUeUsageType 0 [],
        user_location_information_for_sgw: "User Location Information for SGW" CO Uli 1 [],
        sgw_u_node_name: "SGW-U node name" CO Fqdn 0 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" CO* SecondaryRatUsageDataReport 0 [],
        up_function_selection_indication_flags: "UP Function Selection Indication Flags" CO UpFunctionSelectionIndicationFlags 0 [],
        apn_rate_control_status: "APN RATE Control Status" CO ApnRateControlStatus 0 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
        trace_collection_entity_uri: "Trace Collection Entity URI" O Uri 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context to be created within Create Session Request (Table 7.2.1-2).
    group BearerContextToBeCreatedInCreateSessionRequest = 93, "Bearer Context", "7.2.1-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        tft: "TFT" O EpsBearerLevelTrafficFlowTemplate 0 [],
        s1_u_enodeb_f_teid: "S1-U eNodeB F-TEID" CO FTeid 0 [],
        s4_u_sgsn_f_teid: "S4-U SGSN F-TEID" C FTeid 1 [],
        s5_s8_u_sgw_f_teid: "S5/S8-U SGW F-TEID" C FTeid 2 [],
        s5_s8_u_pgw_f_teid: "S5/S8-U PGW F-TEID" CO FTeid 3 [],
        s12_rnc_f_teid: "S12 RNC F-TEID" CO FTeid 4 [],
        s2b_u_epdg_f_teid: "S2b-U ePDG F-TEID" C FTeid 5 [],
        s2a_u_twan_f_teid: "S2a-U TWAN F-TEID" C FTeid 6 [],
        bearer_level_qos: "Bearer Level QoS" M BearerLevelQualityOfService 0 [],
        s11_u_mme_f_teid: "S11-U MME F-TEID" CO FTeid 7 [],
    }
}

gtpv2c_table! {
    /// Bearer Context to be removed within Create Session Request (Table 7.2.1-3).
    group BearerContextToBeRemovedInCreateSessionRequest = 93, "Bearer Context", "7.2.1-3" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        s4_u_sgsn_f_teid: "S4-U SGSN F-TEID" C FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Create Session Request (Table 7.2.1-4).
    group OverloadControlInformationInCreateSessionRequest = 180, "Overload Control Information", "7.2.1-4" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Remote UE Context Connected within Create Session Request (Table 7.2.1-5).
    group RemoteUeContextConnectedInCreateSessionRequest = 191, "Remote UE Context", "7.2.1-5" {
        remote_user_id: "Remote User ID" M RemoteUserId 0 [],
        remote_ue_ip_information: "Remote UE IP Information" M RemoteUeIpInformation 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Create Session Response (Table 7.2.2-1).
    message CreateSessionResponse = 33, "Create Session Response", "7.2.2-1" {
        cause: "Cause" M Cause 0 [],
        change_reporting_action: "Change Reporting Action" C ChangeReportingAction 0 [],
        csg_information_reporting_action: "CSG Information Reporting Action" CO CsgInformationReportingAction 0 [],
        hnb_information_reporting: "H(e)NB Information Reporting" CO HENbInformationReporting 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" C FTeid 0 [],
        pgw_s5_s8_s2a_s2b_f_teid_for_pmip_based_interface_or_for_gtp_based_control_plane_interface: "PGW S5/S8/ S2a/S2b F-TEID for PMIP based interface or for GTP based Control Plane interface" CO FTeid 1 [],
        pdn_address_allocation: "PDN Address Allocation (PAA)" CO Paa 0 [],
        apn_restriction: "APN Restriction" C ApnRestriction 0 [],
        aggregate_maximum_bit_rate: "Aggregate Maximum Bit Rate (APN-AMBR)" C Ambr 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID" C Ebi 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        bearer_contexts_created: "Bearer Contexts created" M* BearerContextCreatedInCreateSessionResponse 0 [],
        bearer_contexts_marked_for_removal: "Bearer Contexts marked for removal" C* BearerContextMarkedForRemovalInACreateSessionResponse 1 [],
        recovery: "Recovery" C Recovery 0 [],
        charging_gateway_name: "Charging Gateway Name" C Fqdn 0 [],
        charging_gateway_address: "Charging Gateway Address" C IpAddress 0 [],
        pgw_fq_csid: "PGW-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        sgw_ldn: "SGW LDN" O Ldn 0 [],
        pgw_ldn: "PGW LDN" O Ldn 1 [],
        pgw_back_off_time: "PGW Back-Off Time" O EpcTimer 0 [],
        additional_protocol_configuration_options: "Additional Protocol Configuration Options (APCO)" O Apco 0 [],
        trusted_wlan_ipv4_parameters: "Trusted WLAN IPv4 Parameters" CO Ip4cp 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        presence_reporting_area_action: "Presence Reporting Area Action" CO* PresenceReportingAreaAction 0 [],
        pgw_s_node_level_load_control_information: "PGW's node level Load Control Information" O LoadControlInformationInCreateSessionResponse 0 [],
        pgw_s_apn_level_load_control_information: "PGW's APN level Load Control Information" O LoadControlInformationInCreateSessionResponse 1 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInCreateSessionResponse 2 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInCreateSessionResponse 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInCreateSessionResponse 1 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        pdn_connection_charging_id: "PDN Connection Charging ID" CO ChargingId 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
        pgw_node_name: "PGW node name" CO Fqdn 1 [],
        sgi_ptp_tunnel_address: "SGi PtP Tunnel Address" CO SgiPtpTunnelAddress 0 [],
        pgw_change_info: "PGW Change Info" CO PgwChangeInfoInCreateSessionResponse 0 [],
        alternative_pgw_c_smf_fqdn: "Alternative PGW-C/SMF FQDN" O* Fqdn 3 [],
        alternative_pgw_c_smf_ip_address: "Alternative PGW-C/SMF IP Address" O* IpAddress 1 [],
        up_security_policy: "UP Security Policy" CO UpSecurityPolicy 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context Created within Create Session Response (Table 7.2.2-2).
    group BearerContextCreatedInCreateSessionResponse = 93, "Bearer Context", "7.2.2-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
        s1_u_sgw_f_teid: "S1-U SGW F-TEID" C FTeid 0 [],
        s4_u_sgw_f_teid: "S4-U SGW F-TEID" C FTeid 1 [],
        s5_s8_u_pgw_f_teid: "S5/S8-U PGW F-TEID" C FTeid 2 [],
        s12_sgw_f_teid: "S12 SGW F-TEID" C FTeid 3 [],
        s2b_u_pgw_f_teid: "S2b-U PGW F-TEID" C FTeid 4 [],
        s2a_u_pgw_f_teid: "S2a-U PGW F-TEID" C FTeid 5 [],
        bearer_level_qos: "Bearer Level QoS" C BearerLevelQualityOfService 0 [],
        charging_id: "Charging Id" O ChargingId 0 [],
        bearer_flags: "Bearer Flags" O BearerFlags 0 [],
        s11_u_sgw_f_teid: "S11-U SGW F-TEID" C FTeid 6 [],
    }
}

gtpv2c_table! {
    /// Bearer Context marked for removal within a Create Session Response (Table 7.2.2-3).
    group BearerContextMarkedForRemovalInACreateSessionResponse = 93, "Bearer Context", "7.2.2-3" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Create Session Response (Table 7.2.2-4).
    group LoadControlInformationInCreateSessionResponse = 181, "Load Control Information", "7.2.2-4" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
        list_of_apn_and_relative_capacity: "List of APN and Relative Capacity" CO ApnAndRelativeCapacity 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Create Session Response (Table 7.2.2-5).
    group OverloadControlInformationInCreateSessionResponse = 180, "Overload Control Information", "7.2.2-5" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// PGW Change Info within Create Session Response (Table 7.2.2-6).
    group PgwChangeInfoInCreateSessionResponse = 214, "PGW Change Info", "7.2.2-6" {
        pgw_set_fqdn: "PGW Set FQDN" C PgwFqdn 0 [],
        alternative_pgw_c_smf_ip_address: "Alternative PGW-C/SMF IP Address" C* IpAddress 0 [],
        alternative_pgw_c_smf_fqdn: "Alternative PGW-C/SMF FQDN" C* PgwFqdn 1 [],
        group_id: "Group Id" O GroupId 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Create Bearer Request (Table 7.2.3-1).
    message CreateBearerRequest = 95, "Create Bearer Request", "7.2.3-1" {
        procedure_transaction_id: "Procedure Transaction Id (PTI)" C ProcedureTransactionId 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID (LBI)" M Ebi 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" O Pco 0 [],
        bearer_contexts: "Bearer Contexts" M* BearerContextInCreateBearerRequest 0 [],
        pgw_fq_csid: "PGW-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        change_reporting_action: "Change Reporting Action" C ChangeReportingAction 0 [],
        csg_information_reporting_action: "CSG Information Reporting Action" CO CsgInformationReportingAction 0 [],
        hnb_information_reporting: "H(e)NB Information Reporting" CO HENbInformationReporting 0 [],
        presence_reporting_area_action: "Presence Reporting Area Action" CO* PresenceReportingAreaAction 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        pgw_s_node_level_load_control_information: "PGW's node level Load Control Information" O LoadControlInformationInCreateBearerRequest 0 [],
        pgw_s_apn_level_load_control_information: "PGW's APN level Load Control Information" O LoadControlInformationInCreateBearerRequest 1 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInCreateBearerRequest 2 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInCreateBearerRequest 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInCreateBearerRequest 1 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        pgw_change_info: "PGW Change Info" CO* PgwChangeInfoInCreateBearerRequest 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" CO FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Create Bearer Request (Table 7.2.3-2).
    group BearerContextInCreateBearerRequest = 93, "Bearer Context", "7.2.3-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        tft: "TFT" M EpsBearerLevelTrafficFlowTemplate 0 [],
        s1_u_sgw_f_teid: "S1-U SGW F-TEID" C FTeid 0 [],
        s5_8_u_pgw_f_teid: "S5/8-U PGW F-TEID" C FTeid 1 [],
        s12_sgw_f_teid: "S12 SGW F-TEID" C FTeid 2 [],
        s4_u_sgw_f_teid: "S4-U SGW F-TEID" C FTeid 3 [],
        s2b_u_pgw_f_teid: "S2b-U PGW F-TEID" C FTeid 4 [],
        s2a_u_pgw_f_teid: "S2a-U PGW F-TEID" C FTeid 5 [],
        bearer_level_qos: "Bearer Level QoS" M BearerLevelQualityOfService 0 [],
        charging_id: "Charging Id" O ChargingId 0 [],
        bearer_flags: "Bearer Flags" O BearerFlags 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" O Pco 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" O ExtendedProtocolConfigurationOptions 0 [],
        maximum_packet_loss_rate: "Maximum Packet Loss Rate" O MaximumPacketLossRate 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Create Bearer Request (Table 7.2.3-3).
    group LoadControlInformationInCreateBearerRequest = 181, "Load Control Information", "7.2.3-3" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
        list_of_apn_and_relative_capacity: "List of APN and Relative Capacity" CO ApnAndRelativeCapacity 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Create Bearer Request (Table 7.2.3-4).
    group OverloadControlInformationInCreateBearerRequest = 180, "Overload Control Information", "7.2.3-4" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// PGW Change Info within Create Bearer Request (Table 7.2.3-5).
    group PgwChangeInfoInCreateBearerRequest = 214, "PGW Change Info", "7.2.3-5" {
        pgw_set_fqdn: "PGW Set FQDN" O PgwFqdn 0 [],
        alternative_pgw_c_smf_ip_address: "Alternative PGW-C/SMF IP Address" O* IpAddress 0 [],
        alternative_pgw_c_smf_fqdn: "Alternative PGW-C/SMF FQDN" O* PgwFqdn 1 [],
        new_pgw_c_smf_ip_address: "New PGW-C/SMF IP Address" C IpAddress 1 [],
        new_sgw_c_ip_address: "New SGW-C IP Address" O IpAddress 3 [],
        pgw_c_smf_fq_csid: "PGW-C/SMF FQ-CSID" C* FqCsid 0 [],
        group_id: "Group Id" C* GroupId 0 [],
        pgw_control_plane_ip_address: "PGW Control Plane IP Address" C* IpAddress 2 [],
        new_group_id: "New Group Id" O GroupId 1 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Create Bearer Response (Table 7.2.4-1).
    message CreateBearerResponse = 96, "Create Bearer Response", "7.2.4-1" {
        cause: "Cause" M Cause 0 [],
        bearer_contexts: "Bearer Contexts" M* BearerContextInCreateBearerResponse 0 [],
        recovery: "Recovery" C Recovery 0 [],
        mme_fq_csid: "MME-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        epdg_fq_csid: "ePDG-FQ-CSID" C FqCsid 2 [],
        twan_fq_csid: "TWAN-FQ-CSID" C FqCsid 3 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" C Pco 0 [],
        ue_time_zone: "UE Time Zone" O UeTimeZone 0 [],
        user_location_information: "User Location Information (ULI)" CO Uli 0 [],
        twan_identifier: "TWAN Identifier" CO TwanIdentifier 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInCreateBearerResponse 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInCreateBearerResponse 1 [],
        presence_reporting_area_information: "Presence Reporting Area Information" CO* PresenceReportingAreaInformation 0 [],
        mme_s4_sgsn_identifier: "MME/S4-SGSN Identifier" CO IpAddress 0 [],
        twan_epdg_s_overload_control_information: "TWAN/ePDG's Overload Control Information" O OverloadControlInformationInCreateBearerResponse 2 [],
        wlan_location_information: "WLAN Location Information" CO TwanIdentifier 1 [],
        wlan_location_timestamp: "WLAN Location Timestamp" CO TwanIdentifierTimestamp 1 [],
        ue_local_ip_address: "UE Local IP Address" CO IpAddress 0 [],
        ue_udp_port: "UE UDP Port" CO PortNumber 0 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        ue_tcp_port: "UE TCP Port" CO PortNumber 1 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Create Bearer Response (Table 7.2.4-2).
    group BearerContextInCreateBearerResponse = 93, "Bearer Context", "7.2.4-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
        s1_u_enodeb_f_teid: "S1-U eNodeB F-TEID" C FTeid 0 [],
        s1_u_sgw_f_teid: "S1-U SGW F-TEID" C FTeid 1 [],
        s5_8_u_sgw_f_teid: "S5/8-U SGW F-TEID" C FTeid 2 [],
        s5_8_u_pgw_f_teid: "S5/8-U PGW F-TEID" C FTeid 3 [],
        s12_rnc_f_teid: "S12 RNC F-TEID" C FTeid 4 [],
        s12_sgw_f_teid: "S12 SGW F-TEID" C FTeid 5 [],
        s4_u_sgsn_f_teid: "S4-U SGSN F-TEID" C FTeid 6 [],
        s4_u_sgw_f_teid: "S4-U SGW F-TEID" C FTeid 7 [],
        s2b_u_epdg_f_teid: "S2b-U ePDG F-TEID" C FTeid 8 [],
        s2b_u_pgw_f_teid: "S2b-U PGW F-TEID" C FTeid 9 [],
        s2a_u_twan_f_teid: "S2a-U TWAN F-TEID" C FTeid 10 [],
        s2a_u_pgw_f_teid: "S2a-U PGW F-TEID" C FTeid 11 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        ran_nas_cause: "RAN/NAS Cause" CO* RanNasCause 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Create Bearer Response (Table 7.2.4-3).
    group OverloadControlInformationInCreateBearerResponse = 180, "Overload Control Information", "7.2.4-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Bearer Resource Command (Table 7.2.5-1).
    message BearerResourceCommand = 68, "Bearer Resource Command", "7.2.5-1" {
        linked_eps_bearer_id: "Linked EPS Bearer ID (LBI)" M Ebi 0 [],
        procedure_transaction_id: "Procedure Transaction Id (PTI)" M ProcedureTransactionId 0 [],
        flow_quality_of_service: "Flow Quality of Service (Flow QoS)" C FlowQualityOfService 0 [],
        traffic_aggregate_description: "Traffic Aggregate Description (TAD)" C Tad 0 [],
        rat_type: "RAT Type" C RatType 0 [],
        serving_network: "Serving Network" O ServingNetwork 0 [],
        user_location_information: "User Location Information (ULI)" O Uli 0 [],
        eps_bearer_id: "EPS Bearer ID" C Ebi 1 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        s4_u_sgsn_f_teid: "S4-U SGSN F-TEID" C FTeid 0 [],
        s12_rnc_f_teid: "S12 RNC F-TEID" C FTeid 1 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" O Pco 0 [],
        signalling_priority_indication: "Signalling Priority Indication" CO SignallingPriorityIndication 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInBearerResourceCommand 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInBearerResourceCommand 1 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" O ExtendedProtocolConfigurationOptions 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" CO FTeid 2 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Bearer Resource Command (Table 7.2.5-2).
    group OverloadControlInformationInBearerResourceCommand = 180, "Overload Control Information", "7.2.5-2" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Bearer Resource Failure Indication (Table 7.2.6-1).
    message BearerResourceFailureIndication = 69, "Bearer Resource Failure Indication", "7.2.6-1" {
        cause: "Cause" M Cause 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID" M Ebi 0 [],
        procedure_transaction_id: "Procedure Transaction ID (PTI)" M ProcedureTransactionId 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInBearerResourceFailureIndication 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInBearerResourceFailureIndication 1 [],
        recovery: "Recovery" O Recovery 0 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Bearer Resource Failure Indication (Table 7.2.6-2).
    group OverloadControlInformationInBearerResourceFailureIndication = 180, "Overload Control Information", "7.2.6-2" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Modify Bearer Request (Table 7.2.7-1).
    message ModifyBearerRequest = 34, "Modify Bearer Request", "7.2.7-1" {
        me_identity: "ME Identity (MEI)" O Mei 0 [],
        user_location_information: "User Location Information (ULI)" CO Uli 0 [],
        serving_network: "Serving Network" CO ServingNetwork 0 [],
        rat_type: "RAT Type" CO RatType 0 [],
        indication_flags: "Indication Flags" C Indication 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" C FTeid 0 [],
        aggregate_maximum_bit_rate: "Aggregate Maximum Bit Rate (APN-AMBR)" C Ambr 0 [],
        delay_downlink_packet_notification_request: "Delay Downlink Packet Notification Request" CO DelayValue 0 [],
        bearer_contexts_to_be_modified: "Bearer Contexts to be modified" C* BearerContextToBeModifiedInModifyBearerRequest 0 [],
        bearer_contexts_to_be_removed: "Bearer Contexts to be removed" C* BearerContextToBeRemovedInModifyBearerRequest 1 [],
        recovery: "Recovery" C Recovery 0 [],
        ue_time_zone: "UE Time Zone" CO UeTimeZone 0 [],
        mme_fq_csid: "MME-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        user_csg_information: "User CSG Information (UCI)" CO Uci 0 [],
        ue_local_ip_address: "UE Local IP Address" CO IpAddress 1 [],
        ue_udp_port: "UE UDP Port" CO PortNumber 1 [],
        mme_s4_sgsn_ldn: "MME/S4-SGSN LDN" O Ldn 0 [],
        sgw_ldn: "SGW LDN" O Ldn 1 [],
        hnb_local_ip_address: "H(e)NB Local IP Address" CO IpAddress 0 [],
        hnb_udp_port: "H(e)NB UDP Port" CO PortNumber 0 [],
        mme_s4_sgsn_identifier: "MME/S4-SGSN Identifier" CO IpAddress 2 [],
        cn_operator_selection_entity: "CN Operator Selection Entity" CO CnOperatorSelectionEntity 0 [],
        presence_reporting_area_information: "Presence Reporting Area Information" CO* PresenceReportingAreaInformation 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInModifyBearerRequest 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInModifyBearerRequest 1 [],
        epdg_s_overload_control_information: "ePDG's Overload Control Information" O OverloadControlInformationInModifyBearerRequest 2 [],
        serving_plmn_rate_control: "Serving PLMN Rate Control" CO ServingPlmnRateControl 0 [],
        mo_exception_data_counter: "MO Exception Data Counter" CO Counter 0 [],
        imsi: "IMSI" O Imsi 0 [],
        user_location_information_for_sgw: "User Location Information for SGW" CO Uli 1 [],
        wlan_location_information: "WLAN Location Information" CO TwanIdentifier 0 [],
        wlan_location_timestamp: "WLAN Location Timestamp" CO TwanIdentifierTimestamp 0 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" CO* SecondaryRatUsageDataReport 0 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context to be modified within Modify Bearer Request (Table 7.2.7-2).
    group BearerContextToBeModifiedInModifyBearerRequest = 93, "Bearer Context", "7.2.7-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        s1_enodeb_f_teid: "S1 eNodeB F-TEID" C FTeid 0 [],
        s5_8_u_sgw_f_teid: "S5/8-U SGW F-TEID" C FTeid 1 [],
        s12_rnc_f_teid: "S12 RNC F-TEID" C FTeid 2 [],
        s4_u_sgsn_f_teid: "S4-U SGSN F-TEID" C FTeid 3 [],
        s11_u_mme_f_teid: "S11-U MME F-TEID" CO FTeid 4 [],
    }
}

gtpv2c_table! {
    /// Bearer Context to be removed within Modify Bearer Request (Table 7.2.7-3).
    group BearerContextToBeRemovedInModifyBearerRequest = 93, "Bearer Context", "7.2.7-3" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Modify Bearer Request (Table 7.2.7-4).
    group OverloadControlInformationInModifyBearerRequest = 180, "Overload Control Information", "7.2.7-4" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Modify Bearer Response (Table 7.2.8-1).
    message ModifyBearerResponse = 35, "Modify Bearer Response", "7.2.8-1" {
        cause: "Cause" M Cause 0 [],
        msisdn: "MSISDN" C Msisdn 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID" C Ebi 0 [],
        apn_restriction: "APN Restriction" C ApnRestriction 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" C Pco 0 [],
        bearer_contexts_modified: "Bearer Contexts modified" C* BearerContextModifiedInModifyBearerResponse 0 [],
        bearer_contexts_marked_for_removal: "Bearer Contexts marked for removal" C* BearerContextMarkedForRemovalInModifyBearerResponse 1 [],
        change_reporting_action: "Change Reporting Action" C ChangeReportingAction 0 [],
        csg_information_reporting_action: "CSG Information Reporting Action" CO CsgInformationReportingAction 0 [],
        hnb_information_reporting: "H(e)NB Information Reporting" CO HENbInformationReporting 0 [],
        charging_gateway_name: "Charging Gateway Name" C Fqdn 0 [],
        charging_gateway_address: "Charging Gateway Address" C IpAddress 0 [],
        pgw_fq_csid: "PGW-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        recovery: "Recovery" C Recovery 0 [],
        sgw_ldn: "SGW LDN" O Ldn 0 [],
        pgw_ldn: "PGW LDN" O Ldn 1 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        presence_reporting_area_action: "Presence Reporting Area Action" CO* PresenceReportingAreaAction 0 [],
        pgw_s_node_level_load_control_information: "PGW's node level Load Control Information" O LoadControlInformationInModifyBearerResponse 0 [],
        pgw_s_apn_level_load_control_information: "PGW's APN level Load Control Information" O LoadControlInformationInModifyBearerResponse 1 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInModifyBearerResponse 2 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInModifyBearerResponse 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInModifyBearerResponse 1 [],
        pdn_connection_charging_id: "PDN Connection Charging ID" CO ChargingId 0 [],
        pgw_change_info: "PGW Change Info" CO PgwChangeInfoInModifyBearerResponse 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context modified within Modify Bearer Response (Table 7.2.8-2).
    group BearerContextModifiedInModifyBearerResponse = 93, "Bearer Context", "7.2.8-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
        s1_u_sgw_f_teid: "S1-U SGW F-TEID" C FTeid 0 [],
        s12_sgw_f_teid: "S12 SGW F-TEID" C FTeid 1 [],
        s4_u_sgw_f_teid: "S4-U SGW F-TEID" C FTeid 2 [],
        charging_id: "Charging ID" O ChargingId 0 [],
        bearer_flags: "Bearer Flags" CO BearerFlags 0 [],
        s11_u_sgw_f_teid: "S11-U SGW F-TEID" C FTeid 3 [],
    }
}

gtpv2c_table! {
    /// Bearer Context marked for removal within Modify Bearer Response (Table 7.2.8-3).
    group BearerContextMarkedForRemovalInModifyBearerResponse = 93, "Bearer Context", "7.2.8-3" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Modify Bearer Response (Table 7.2.8-4).
    group LoadControlInformationInModifyBearerResponse = 181, "Load Control Information", "7.2.8-4" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
        list_of_apn_and_relative_capacity: "List of APN and Relative Capacity" CO ApnAndRelativeCapacity 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Modify Bearer Response (Table 7.2.8-5).
    group OverloadControlInformationInModifyBearerResponse = 180, "Overload Control Information", "7.2.8-5" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// PGW Change Info within Modify Bearer Response (Table 7.2.8-6).
    group PgwChangeInfoInModifyBearerResponse = 214, "PGW Change Info", "7.2.8-6" {
        pgw_set_fqdn: "PGW Set FQDN" C PgwFqdn 0 [],
        alternative_pgw_c_smf_ip_address: "Alternative PGW-C/SMF IP Address" C* IpAddress 0 [],
        alternative_pgw_c_smf_fqdn: "Alternative PGW-C/SMF FQDN" C* PgwFqdn 1 [],
        group_id: "Group Id" O GroupId 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Delete Session Request (Table 7.2.9.1-1).
    message DeleteSessionRequest = 36, "Delete Session Request", "7.2.9.1-1" {
        cause: "Cause" CO Cause 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID (LBI)" C Ebi 0 [],
        user_location_information: "User Location Information (ULI)" CO Uli 0 [],
        indication_flags: "Indication Flags" C Indication 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        originating_node: "Originating Node" C NodeType 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" O FTeid 0 [],
        ue_time_zone: "UE Time Zone" CO UeTimeZone 0 [],
        uli_timestamp: "ULI Timestamp" CO UliTimestamp 0 [],
        ran_nas_release_cause: "RAN/NAS Release Cause" CO RanNasCause 0 [],
        twan_identifier: "TWAN Identifier" CO TwanIdentifier 0 [],
        twan_identifier_timestamp: "TWAN Identifier Timestamp" CO TwanIdentifierTimestamp 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInDeleteSessionRequest 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInDeleteSessionRequest 1 [],
        twan_epdg_s_overload_control_information: "TWAN/ePDG's Overload Control Information" O OverloadControlInformationInDeleteSessionRequest 2 [],
        wlan_location_information: "WLAN Location Information" CO TwanIdentifier 1 [],
        wlan_location_timestamp: "WLAN Location Timestamp" CO TwanIdentifierTimestamp 1 [],
        ue_local_ip_address: "UE Local IP Address" CO IpAddress 0 [],
        ue_udp_port: "UE UDP Port" CO PortNumber 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
        ue_tcp_port: "UE TCP Port" CO PortNumber 1 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" O* SecondaryRatUsageDataReport 0 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Delete Session Request (Table 7.2.9.1-2).
    group OverloadControlInformationInDeleteSessionRequest = 180, "Overload Control Information", "7.2.9.1-2" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Delete Bearer Request (Table 7.2.9.2-1).
    message DeleteBearerRequest = 99, "Delete Bearer Request", "7.2.9.2-1" {
        linked_eps_bearer_id: "Linked EPS Bearer ID (LBI)" CO* Ebi 0 [],
        eps_bearer_ids: "EPS Bearer IDs" C* Ebi 1 [],
        failed_bearer_contexts: "Failed Bearer Contexts" O BearerContextInDeleteBearerRequest 0 [],
        procedure_transaction_id: "Procedure Transaction Id (PTI)" C ProcedureTransactionId 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        pgw_fq_csid: "PGW-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        cause: "Cause" O Cause 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        pgw_s_node_level_load_control_information: "PGW's node level Load Control Information" O LoadControlInformationInDeleteBearerRequest 0 [],
        pgw_s_apn_level_load_control_information: "PGW's APN level Load Control Information" O LoadControlInformationInDeleteBearerRequest 1 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInDeleteBearerRequest 2 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInDeleteBearerRequest 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInDeleteBearerRequest 1 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        apn_rate_control_status: "APN RATE Control Status" CO ApnRateControlStatus 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
        pgw_change_info: "PGW Change Info" CO* PgwChangeInfoInDeleteBearerRequest 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" CO FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Delete Bearer Request (Table 7.2.9.2-2).
    group BearerContextInDeleteBearerRequest = 93, "Bearer Context", "7.2.9.2-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Delete Bearer Request (Table 7.2.9.2-3).
    group LoadControlInformationInDeleteBearerRequest = 181, "Load Control Information", "7.2.9.2-3" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
        list_of_apn_and_relative_capacity: "List of APN and Relative Capacity" CO ApnAndRelativeCapacity 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Delete Bearer Request (Table 7.2.9.2-4).
    group OverloadControlInformationInDeleteBearerRequest = 180, "Overload Control Information", "7.2.9.2-4" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// PGW Change Info within Delete Bearer Request (Table 7.2.9.2-5).
    group PgwChangeInfoInDeleteBearerRequest = 214, "PGW Change Info", "7.2.9.2-5" {
        pgw_set_fqdn: "PGW Set FQDN" O PgwFqdn 0 [],
        alternative_pgw_c_smf_ip_address: "Alternative PGW-C/SMF IP Address" O* IpAddress 0 [],
        alternative_pgw_c_smf_fqdn: "Alternative PGW-C/SMF FQDN" O* PgwFqdn 1 [],
        new_pgw_c_smf_ip_address: "New PGW-C/SMF IP Address" C IpAddress 1 [],
        new_sgw_c_ip_address: "New SGW-C IP Address" O IpAddress 3 [],
        pgw_c_smf_fq_csid: "PGW-C/SMF FQ-CSID" C* FqCsid 0 [],
        group_id: "Group Id" C* GroupId 0 [],
        pgw_control_plane_ip_address: "PGW Control Plane IP Address" C* IpAddress 2 [],
        new_group_id: "New Group Id" O GroupId 1 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Delete Session Response (Table 7.2.10.1-1).
    message DeleteSessionResponse = 37, "Delete Session Response", "7.2.10.1-1" {
        cause: "Cause" M Cause 0 [],
        recovery: "Recovery" C Recovery 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        pgw_s_node_level_load_control_information: "PGW's node level Load Control Information" O LoadControlInformationInDeleteSessionResponse 0 [],
        pgw_s_apn_level_load_control_information: "PGW's APN level Load Control Information" O LoadControlInformationInDeleteSessionResponse 1 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInDeleteSessionResponse 2 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInDeleteSessionResponse 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInDeleteSessionResponse 1 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
        apn_rate_control_status: "APN RATE Control Status" CO ApnRateControlStatus 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Delete Session Response (Table 7.2.10.1-2).
    group LoadControlInformationInDeleteSessionResponse = 181, "Load Control Information", "7.2.10.1-2" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
        list_of_apn_and_relative_capacity: "List of APN and Relative Capacity" CO ApnAndRelativeCapacity 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Delete Session Response (Table 7.2.10.1-3).
    group OverloadControlInformationInDeleteSessionResponse = 180, "Overload Control Information", "7.2.10.1-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in Delete Bearer Response (Table 7.2.10.2-1).
    message DeleteBearerResponse = 100, "Delete Bearer Response", "7.2.10.2-1" {
        cause: "Cause" M Cause 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID (LBI)" CO* Ebi 0 [],
        bearer_contexts: "Bearer Contexts" C* BearerContextInDeleteBearerResponse 0 [],
        recovery: "Recovery" C Recovery 0 [],
        mme_fq_csid: "MME-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        epdg_fq_csid: "ePDG-FQ-CSID" C FqCsid 2 [],
        twan_fq_csid: "TWAN-FQ-CSID" C FqCsid 3 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        ue_time_zone: "UE Time Zone" CO UeTimeZone 0 [],
        user_location_information: "User Location Information (ULI)" CO Uli 0 [],
        uli_timestamp: "ULI Timestamp" CO UliTimestamp 0 [],
        twan_identifier: "TWAN Identifier" CO TwanIdentifier 0 [],
        twan_identifier_timestamp: "TWAN Identifier Timestamp" CO TwanIdentifierTimestamp 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInDeleteBearerResponse 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInDeleteBearerResponse 1 [],
        mme_s4_sgsn_identifier: "MME/S4-SGSN Identifier" CO IpAddress 0 [],
        twan_epdg_s_overload_control_information: "TWAN/ePDG's Overload Control Information" O OverloadControlInformationInDeleteBearerResponse 2 [],
        wlan_location_information: "WLAN Location Information" CO TwanIdentifier 1 [],
        wlan_location_timestamp: "WLAN Location Timestamp" CO TwanIdentifierTimestamp 1 [],
        ue_local_ip_address: "UE Local IP Address" CO IpAddress 0 [],
        ue_udp_port: "UE UDP Port" CO PortNumber 0 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        ue_tcp_port: "UE TCP Port" CO PortNumber 1 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" O* SecondaryRatUsageDataReport 0 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Delete Bearer Response (Table 7.2.10.2-2).
    group BearerContextInDeleteBearerResponse = 93, "Bearer Context", "7.2.10.2-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        ran_nas_cause: "RAN/NAS Cause" CO* RanNasCause 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Delete Bearer Response (Table 7.2.10.2-3).
    group OverloadControlInformationInDeleteBearerResponse = 180, "Overload Control Information", "7.2.10.2-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Downlink Data Notification (Table 7.2.11.1-1).
    message DownlinkDataNotification = 176, "Downlink Data Notification", "7.2.11.1-1" {
        cause: "Cause" CO Cause 0 [],
        eps_bearer_id: "EPS Bearer ID" CO* Ebi 0 [],
        allocation_retention_priority: "Allocation/Retention Priority" CO Arp 0 [],
        imsi: "IMSI" CO Imsi 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" O FTeid 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInDownlinkDataNotification 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInDownlinkDataNotification 0 [],
        paging_and_service_information: "Paging and Service Information" CO PagingAndServiceInformation 0 [],
        dl_data_packets_size: "DL Data Packets Size" CO IntegerNumber 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Downlink Data Notification (Table 7.2.11.1-2).
    group LoadControlInformationInDownlinkDataNotification = 181, "Load Control Information", "7.2.11.1-2" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Downlink Data Notification (Table 7.2.11.1-3).
    group OverloadControlInformationInDownlinkDataNotification = 180, "Overload Control Information", "7.2.11.1-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Downlink Data Notification Acknowledge (Table 7.2.11.2-1).
    message DownlinkDataNotificationAcknowledge = 177, "Downlink Data Notification Acknowledge", "7.2.11.2-1" {
        cause: "Cause" M Cause 0 [],
        data_notification_delay: "Data Notification Delay" C DelayValue 0 [],
        recovery: "Recovery" C Recovery 0 [],
        dl_low_priority_traffic_throttling: "DL low priority traffic Throttling" O Throttling 0 [],
        imsi: "IMSI" CO Imsi 0 [],
        dl_buffering_duration: "DL Buffering Duration" CO EpcTimer 0 [],
        dl_buffering_suggested_packet_count: "DL Buffering Suggested Packet Count" O IntegerNumber 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Downlink Data Notification Failure Indication (Table 7.2.11.3-1).
    message DownlinkDataNotificationFailureIndication = 70, "Downlink Data Notification Failure Indication", "7.2.11.3-1" {
        cause: "Cause" M Cause 0 [],
        originating_node: "Originating Node" CO NodeType 0 [],
        imsi: "IMSI" CO Imsi 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Delete Indirect Data Forwarding Tunnel Request (Table 7.2.12-1).
    message DeleteIndirectDataForwardingTunnelRequest = 168, "Delete Indirect Data Forwarding Tunnel Request", "7.2.12-1" {
    }
}

gtpv2c_table! {
    /// Information Element in Delete Indirect Data Forwarding Tunnel Response (Table 7.2.13-1).
    message DeleteIndirectDataForwardingTunnelResponse = 169, "Delete Indirect Data Forwarding Tunnel Response", "7.2.13-1" {
        cause: "Cause" M Cause 0 [],
        recovery: "Recovery" C Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Modify Bearer Command (Table 7.2.14.1-1).
    message ModifyBearerCommand = 64, "Modify Bearer Command", "7.2.14.1-1" {
        apn_aggregate_maximum_bit_rate: "APN-Aggregate Maximum Bit Rate (APN-AMBR)" M Ambr 0 [],
        bearer_context: "Bearer Context" M BearerContextInModifyBearerCommand 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInModifyBearerCommand 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInModifyBearerCommand 1 [],
        twan_epdg_s_overload_control_information: "TWAN/ePDG's Overload Control Information" O OverloadControlInformationInModifyBearerCommand 2 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" CO FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Modify Bearer Command (Table 7.2.14.1-2).
    group BearerContextInModifyBearerCommand = 93, "Bearer Context", "7.2.14.1-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        bearer_level_qos: "Bearer Level QoS" CO BearerLevelQualityOfService 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Modify Bearer Command (Table 7.2.14-3).
    group OverloadControlInformationInModifyBearerCommand = 180, "Overload Control Information", "7.2.14-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Modify Bearer Failure Indication (Table 7.2.14.2-1).
    message ModifyBearerFailureIndication = 65, "Modify Bearer Failure Indication", "7.2.14.2-1" {
        cause: "Cause" M Cause 0 [],
        recovery: "Recovery" C Recovery 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInModifyBearerFailureIndication 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInModifyBearerFailureIndication 1 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Modify Bearer Failure Indication (Table 7.2.14-2).
    group OverloadControlInformationInModifyBearerFailureIndication = 180, "Overload Control Information", "7.2.14-2" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in an Update Bearer Request (Table 7.2.15-1).
    message UpdateBearerRequest = 97, "Update Bearer Request", "7.2.15-1" {
        bearer_contexts: "Bearer Contexts" M* BearerContextInUpdateBearerRequest 0 [],
        procedure_transaction_id: "Procedure Transaction Id (PTI)" C ProcedureTransactionId 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        aggregate_maximum_bit_rate: "Aggregate Maximum Bit Rate (APN-AMBR)" M Ambr 0 [],
        change_reporting_action: "Change Reporting Action" C ChangeReportingAction 0 [],
        csg_information_reporting_action: "CSG Information Reporting Action" CO CsgInformationReportingAction 0 [],
        hnb_information_reporting: "H(e)NB Information Reporting" CO HENbInformationReporting 0 [],
        indication_flags: "Indication flags" CO Indication 0 [],
        pgw_fq_csid: "PGW-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        presence_reporting_area_action: "Presence Reporting Area Action" CO* PresenceReportingAreaAction 0 [],
        pgw_s_node_level_load_control_information: "PGW's node level Load Control Information" O LoadControlInformationInUpdateBearerRequest 0 [],
        pgw_s_apn_level_load_control_information: "PGW's APN level Load Control Information" O LoadControlInformationInUpdateBearerRequest 1 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInUpdateBearerRequest 2 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInUpdateBearerRequest 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInUpdateBearerRequest 1 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        pgw_change_info: "PGW Change Info" CO* PgwChangeInfoInUpdateBearerRequest 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" CO FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Update Bearer Request (Table 7.2.15-2).
    group BearerContextInUpdateBearerRequest = 93, "Bearer Context", "7.2.15-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        tft: "TFT" C EpsBearerLevelTrafficFlowTemplate 0 [],
        bearer_level_qos: "Bearer Level QoS" C BearerLevelQualityOfService 0 [],
        bearer_flags: "Bearer Flags" O BearerFlags 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        additional_protocol_configuration_options: "Additional Protocol Configuration Options (APCO)" CO Apco 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
        maximum_packet_loss_rate: "Maximum Packet Loss Rate" O MaximumPacketLossRate 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Update Bearer Request (Table 7.2.15-3).
    group LoadControlInformationInUpdateBearerRequest = 181, "Load Control Information", "7.2.15-3" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
        list_of_apn_and_relative_capacity: "List of APN and Relative Capacity" CO ApnAndRelativeCapacity 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Update Bearer Request (Table 7.2.15-4).
    group OverloadControlInformationInUpdateBearerRequest = 180, "Overload Control Information", "7.2.15-4" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// PGW Change Info within Update Bearer Request (Table 7.2.15-5).
    group PgwChangeInfoInUpdateBearerRequest = 214, "PGW Change Info", "7.2.15-5" {
        pgw_set_fqdn: "PGW Set FQDN" O PgwFqdn 0 [],
        alternative_pgw_c_smf_ip_address: "Alternative PGW-C/SMF IP Address" O* IpAddress 0 [],
        alternative_pgw_c_smf_fqdn: "Alternative PGW-C/SMF FQDN" O* PgwFqdn 1 [],
        new_pgw_c_smf_ip_address: "New PGW-C/SMF IP Address" C IpAddress 1 [],
        new_sgw_c_ip_address: "New SGW-C IP Address" O IpAddress 3 [],
        pgw_c_smf_fq_csid: "PGW-C/SMF FQ-CSID" C* FqCsid 0 [],
        group_id: "Group Id" C* GroupId 0 [],
        pgw_control_plane_ip_address: "PGW Control Plane IP Address" C* IpAddress 2 [],
        new_group_id: "New Group Id" O GroupId 1 [],
    }
}

gtpv2c_table! {
    /// Information Elements in an Update Bearer Response (Table 7.2.16-1).
    message UpdateBearerResponse = 98, "Update Bearer Response", "7.2.16-1" {
        cause: "Cause" M Cause 0 [],
        bearer_contexts: "Bearer Contexts" M* BearerContextInUpdateBearerResponse 0 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        recovery: "Recovery" C Recovery 0 [],
        mme_fq_csid: "MME-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        epdg_fq_csid: "ePDG-FQ-CSID" C FqCsid 2 [],
        twan_fq_csid: "TWAN-FQ-CSID" C FqCsid 3 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        ue_time_zone: "UE Time Zone" O UeTimeZone 0 [],
        user_location_information: "User Location Information (ULI)" CO Uli 0 [],
        twan_identifier: "TWAN Identifier" CO TwanIdentifier 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInUpdateBearerResponse 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInUpdateBearerResponse 1 [],
        presence_reporting_area_information: "Presence Reporting Area Information" CO* PresenceReportingAreaInformation 0 [],
        mme_s4_sgsn_identifier: "MME/S4-SGSN Identifier" CO IpAddress 0 [],
        twan_epdg_s_overload_control_information: "TWAN/ePDG's Overload Control Information" O OverloadControlInformationInUpdateBearerResponse 2 [],
        wlan_location_information: "WLAN Location Information" CO TwanIdentifier 1 [],
        wlan_location_timestamp: "WLAN Location Timestamp" CO TwanIdentifierTimestamp 1 [],
        ue_local_ip_address: "UE Local IP Address" CO IpAddress 0 [],
        ue_udp_port: "UE UDP Port" CO PortNumber 0 [],
        nbifom_container: "NBIFOM Container" CO FContainer 0 [],
        ue_tcp_port: "UE TCP Port" CO PortNumber 1 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Update Bearer Response (Table 7.2.16-2).
    group BearerContextInUpdateBearerResponse = 93, "Bearer Context", "7.2.16-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
        s4_u_sgsn_f_teid: "S4-U SGSN F-TEID" C FTeid 0 [],
        s12_rnc_f_teid: "S12 RNC F-TEID" C FTeid 1 [],
        protocol_configuration_options: "Protocol Configuration Options (PCO)" CO Pco 0 [],
        ran_nas_cause: "RAN/NAS Cause" CO* RanNasCause 0 [],
        extended_protocol_configuration_options: "Extended Protocol Configuration Options (ePCO)" CO ExtendedProtocolConfigurationOptions 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Update Bearer Response (Table 7.2.16-3).
    group OverloadControlInformationInUpdateBearerResponse = 180, "Overload Control Information", "7.2.16-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in Delete Bearer Command (Table 7.2.17.1-1).
    message DeleteBearerCommand = 66, "Delete Bearer Command", "7.2.17.1-1" {
        bearer_contexts: "Bearer Contexts" M* BearerContextInDeleteBearerCommand 0 [],
        user_location_information: "User Location Information (ULI)" CO Uli 0 [],
        uli_timestamp: "ULI Timestamp" CO UliTimestamp 0 [],
        ue_time_zone: "UE Time Zone" CO UeTimeZone 0 [],
        mme_s4_sgsn_s_overload_control_information: "MME/S4-SGSN's Overload Control Information" O OverloadControlInformationInDeleteBearerCommand 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInDeleteBearerCommand 1 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" CO FTeid 0 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" O* SecondaryRatUsageDataReport 0 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Delete Bearer Command (Table 7.2.17.1-2).
    group BearerContextInDeleteBearerCommand = 93, "Bearer Context", "7.2.17.1-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        bearer_flags: "Bearer Flags" CO BearerFlags 0 [],
        ran_nas_release_cause: "RAN/NAS Release Cause" CO* RanNasCause 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Delete Bearer Command (Table 7.2.17.1-3).
    group OverloadControlInformationInDeleteBearerCommand = 180, "Overload Control Information", "7.2.17.1-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Delete Bearer Failure Indication (Table 7.2.17.2-1).
    message DeleteBearerFailureIndication = 67, "Delete Bearer Failure Indication", "7.2.17.2-1" {
        cause: "Cause" M Cause 0 [],
        bearer_context: "Bearer Context" M* BearerContextInDeleteBearerFailureIndication 0 [],
        recovery: "Recovery" C Recovery 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        pgw_s_overload_control_information: "PGW's Overload Control Information" O OverloadControlInformationInDeleteBearerFailureIndication 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInDeleteBearerFailureIndication 1 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Delete Bearer Failure Indication (Table 7.2.17.2-2).
    group BearerContextInDeleteBearerFailureIndication = 93, "Bearer Context", "7.2.17.2-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Delete Bearer Failure Indication (Table 7.2.17-3).
    group OverloadControlInformationInDeleteBearerFailureIndication = 180, "Overload Control Information", "7.2.17-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
        list_of_access_point_name: "List of Access Point Name (APN)" CO Apn 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Create Indirect Data Forwarding Tunnel Request (Table 7.2.18-1).
    message CreateIndirectDataForwardingTunnelRequest = 166, "Create Indirect Data Forwarding Tunnel Request", "7.2.18-1" {
        imsi: "IMSI" C Imsi 0 [],
        me_identity: "ME Identity (MEI)" C Mei 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" C FTeid 0 [],
        bearer_contexts: "Bearer Contexts" M* BearerContextInCreateIndirectDataForwardingTunnelRequest 0 [],
        recovery: "Recovery" CO Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Create Indirect Data Forwarding Tunnel Request (Table 7.2.18-2).
    group BearerContextInCreateIndirectDataForwardingTunnelRequest = 93, "Bearer Context", "7.2.18-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        enodeb_f_teid_for_dl_data_forwarding: "eNodeB F-TEID for DL data forwarding" CO FTeid 0 [],
        sgw_upf_f_teid_for_dl_data_forwarding: "SGW/UPF F-TEID for DL data forwarding" CO FTeid 1 [],
        sgsn_f_teid_for_dl_data_forwarding: "SGSN F-TEID for DL data forwarding" CO FTeid 2 [],
        rnc_f_teid_for_dl_data_forwarding: "RNC F-TEID for DL data forwarding" CO FTeid 3 [],
        enodeb_f_teid_for_ul_data_forwarding: "eNodeB F-TEID for UL data forwarding" O FTeid 4 [],
        sgw_f_teid_for_ul_data_forwarding: "SGW F-TEID for UL data forwarding" O FTeid 5 [],
        mme_f_teid_for_dl_data_forwarding: "MME F-TEID for DL data forwarding" CO FTeid 6 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Create Indirect Data Forwarding Tunnel Response (Table 7.2.19-1).
    message CreateIndirectDataForwardingTunnelResponse = 167, "Create Indirect Data Forwarding Tunnel Response", "7.2.19-1" {
        cause: "Cause" M Cause 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" C FTeid 0 [],
        bearer_contexts: "Bearer Contexts" M* BearerContextInCreateIndirectDataForwardingTunnelResponse 0 [],
        recovery: "Recovery" CO Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Create Indirect Data Forwarding Tunnel Response (Table 7.2.19-2).
    group BearerContextInCreateIndirectDataForwardingTunnelResponse = 93, "Bearer Context", "7.2.19-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
        s1_u_sgw_f_teid_for_dl_data_forwarding: "S1-U SGW F-TEID for DL data forwarding" C FTeid 0 [],
        s12_sgw_f_teid_for_dl_data_forwarding: "S12 SGW F-TEID for DL data forwarding" C FTeid 1 [],
        s4_u_sgw_f_teid_for_dl_data_forwarding: "S4-U SGW F-TEID for DL data forwarding" C FTeid 2 [],
        sgw_f_teid_for_dl_data_forwarding: "SGW F-TEID for DL data forwarding" C FTeid 3 [],
        s1_u_sgw_f_teid_for_ul_data_forwarding: "S1-U SGW F-TEID for UL data forwarding" O FTeid 4 [],
        sgw_f_teid_for_ul_data_forwarding: "SGW F-TEID for UL data forwarding" O FTeid 5 [],
    }
}

gtpv2c_table! {
    /// Information Element in Release Access Bearers Request (Table 7.2.21-1).
    message ReleaseAccessBearersRequest = 170, "Release Access Bearers Request", "7.2.21-1" {
        list_of_rabs: "List of RABs" C* Ebi 0 [],
        originating_node: "Originating Node" CO NodeType 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" CO* SecondaryRatUsageDataReport 0 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Release Access Bearers Response (Table 7.2.22-1).
    message ReleaseAccessBearersResponse = 171, "Release Access Bearers Response", "7.2.22-1" {
        cause: "Cause" M Cause 0 [],
        recovery: "Recovery" O Recovery 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInReleaseAccessBearersResponse 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInReleaseAccessBearersResponse 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Release Access Bearers Response (Table 7.2.22-2).
    group LoadControlInformationInReleaseAccessBearersResponse = 181, "Load Control Information", "7.2.22-2" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Release Access Bearers Response (Table 7.2.22-3).
    group OverloadControlInformationInReleaseAccessBearersResponse = 180, "Overload Control Information", "7.2.22-3" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Stop Paging Indication (Table 7.2.23-1).
    message StopPagingIndication = 73, "Stop Paging Indication", "7.2.23-1" {
        imsi: "IMSI" CO Imsi 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Modify Access Bearers Request (Table 7.2.24-1).
    message ModifyAccessBearersRequest = 211, "Modify Access Bearers Request", "7.2.24-1" {
        indication_flags: "Indication Flags" C Indication 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" C FTeid 0 [],
        delay_downlink_packet_notification_request: "Delay Downlink Packet Notification Request" C DelayValue 0 [],
        bearer_contexts_to_be_modified: "Bearer Contexts to be modified" C* BearerContextToBeModifiedInModifyAccessBearersRequest 0 [],
        bearer_contexts_to_be_removed: "Bearer Contexts to be removed" C* BearerContextToBeRemovedInModifyAccessBearersRequest 1 [],
        recovery: "Recovery" C Recovery 0 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" CO* SecondaryRatUsageDataReport 0 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context to be modified within Modify Access Bearers Request (Table 7.2.24-2).
    group BearerContextToBeModifiedInModifyAccessBearersRequest = 93, "Bearer Context", "7.2.24-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        s1_u_enodeb_f_teid: "S1-U eNodeB F-TEID" C FTeid 0 [],
        s11_u_mme_f_teid: "S11-U MME F-TEID" CO FTeid 1 [],
    }
}

gtpv2c_table! {
    /// Bearer Context to be removed within Modify Access Bearers Request (Table 7.2.24-3).
    group BearerContextToBeRemovedInModifyAccessBearersRequest = 93, "Bearer Context", "7.2.24-3" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Modify Access Bearers Response (Table 7.2.25-1).
    message ModifyAccessBearersResponse = 212, "Modify Access Bearers Response", "7.2.25-1" {
        cause: "Cause" M Cause 0 [],
        bearer_contexts_modified: "Bearer Contexts modified" C* BearerContextModifiedInModifyAccessBearersResponse 0 [],
        bearer_contexts_marked_for_removal: "Bearer Contexts marked for removal" C* BearerContextMarkedForRemovalInModifyAccessBearersResponse 1 [],
        recovery: "Recovery" C Recovery 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        sgw_s_node_level_load_control_information: "SGW's node level Load Control Information" O LoadControlInformationInModifyAccessBearersResponse 0 [],
        sgw_s_overload_control_information: "SGW's Overload Control Information" O OverloadControlInformationInModifyAccessBearersResponse 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context modified within Modify Access Bearers Response (Table 7.2.25-2).
    group BearerContextModifiedInModifyAccessBearersResponse = 93, "Bearer Context", "7.2.25-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
        s1_u_sgw_f_teid: "S1-U SGW F-TEID" C FTeid 0 [],
        s11_u_sgw_f_teid: "S11-U SGW F-TEID" C FTeid 1 [],
    }
}

gtpv2c_table! {
    /// Bearer Context marked for removal within Modify Access Bearers Response (Table 7.2.25-3).
    group BearerContextMarkedForRemovalInModifyAccessBearersResponse = 93, "Bearer Context", "7.2.25-3" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Load Control Information within Modify Access Bearers Response (Table 7.2.25-4).
    group LoadControlInformationInModifyAccessBearersResponse = 181, "Load Control Information", "7.2.25-4" {
        load_control_sequence_number: "Load Control Sequence Number" M SequenceNumber 0 [],
        load_metric: "Load Metric" M Metric 0 [],
    }
}

gtpv2c_table! {
    /// Overload Control Information within Modify Access Bearers Response (Table 7.2.25-5).
    group OverloadControlInformationInModifyAccessBearersResponse = 180, "Overload Control Information", "7.2.25-5" {
        overload_control_sequence_number: "Overload Control Sequence Number" M SequenceNumber 0 [],
        overload_reduction_metric: "Overload Reduction Metric" M Metric 0 [],
        period_of_validity: "Period of Validity" M EpcTimer 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in Remote UE Report Notification (Table 7.2.26-1).
    message RemoteUeReportNotification = 40, "Remote UE Report Notification", "7.2.26-1" {
        remote_ue_context_connected: "Remote UE Context Connected" C* RemoteUeContextConnectedInRemoteUeReportNotification 0 [],
        remote_ue_context_disconnected: "Remote UE Context Disconnected" C* RemoteUeContextDisconnectedInRemoteUeReportNotification 1 [],
    }
}

gtpv2c_table! {
    /// Remote UE Context Connected within Remote UE Report Notification (Table 7.2.26-2).
    group RemoteUeContextConnectedInRemoteUeReportNotification = 191, "Remote UE Context", "7.2.26-2" {
        remote_user_id: "Remote User ID" M RemoteUserId 0 [],
        remote_ue_ip_information: "Remote UE IP Information" M RemoteUeIpInformation 0 [],
    }
}

gtpv2c_table! {
    /// Remote UE Context Disconnected with Remote UE Report Notification (Table 7.2.26-3).
    group RemoteUeContextDisconnectedInRemoteUeReportNotification = 191, "Remote UE Context", "7.2.26-3" {
        remote_user_id: "Remote User ID" M RemoteUserId 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in Remote UE Report Acknowledge (Table 7.2.27-1).
    message RemoteUeReportAcknowledge = 41, "Remote UE Report Acknowledge", "7.2.27-1" {
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Forward Relocation Request (Table 7.3.1-1).
    message ForwardRelocationRequest = 133, "Forward Relocation Request", "7.3.1-1" {
        imsi: "IMSI" C Imsi 0 [],
        sender_s_f_teid_for_control_plane: "Sender's F-TEID for Control Plane" M FTeid 0 [],
        mme_sgsn_amf_ue_eps_pdn_connections: "MME/SGSN/AMF UE EPS PDN Connections" C* MmeSgsnAmfUeEpsPdnConnectionsInForwardRelocationRequest 0 [],
        sgw_s11_s4_ip_address_and_teid_for_control_plane: "SGW S11/S4 IP Address and TEID for Control Plane" C FTeid 1 [],
        sgw_node_name: "SGW node name" C Fqdn 0 [],
        mme_sgsn_amf_ue_mm_context: "MME/SGSN/AMF UE MM Context" M MmContextGsmKeyAndTriplets 0 [],
        indication_flags: "Indication Flags" C Indication 0 [],
        e_utran_transparent_container: "E-UTRAN Transparent Container" C FContainer 0 [],
        utran_transparent_container: "UTRAN Transparent Container" C FContainer 1 [],
        bss_container: "BSS Container" C FContainer 2 [],
        target_identification: "Target Identification" CO TargetIdentification 0 [],
        hrpd_access_node_s101_ip_address: "HRPD access node S101 IP address" C IpAddress 0 [],
        n1xiws_s102_ip_address: "1xIWS S102 IP address" C IpAddress 1 [],
        s1_ap_cause: "S1-AP Cause" C FCause 0 [],
        ranap_cause: "RANAP Cause" C FCause 1 [],
        bssgp_cause: "BSSGP Cause" C FCause 2 [],
        source_identification: "Source Identification" C SourceIdentification 0 [],
        selected_plmn_id: "Selected PLMN ID" C PlmnId 0 [],
        recovery: "Recovery" C Recovery 0 [],
        trace_information: "Trace Information" C TraceInformation 0 [],
        subscribed_rfsp_index: "Subscribed RFSP Index" CO RfspIndex 0 [],
        rfsp_index_in_use: "RFSP Index in Use" CO RfspIndex 1 [],
        rfsp_index_in_use_validity_time: "RFSP Index in Use Validity Time" O TimerInSeconds 0 [],
        csg_id: "CSG ID" CO CsgId 0 [],
        csg_membership_indication: "CSG Membership Indication" CO Cmi 0 [],
        ue_time_zone: "UE Time Zone" CO UeTimeZone 0 [],
        serving_network: "Serving Network" CO ServingNetwork 0 [],
        mme_s4_sgsn_ldn: "MME/S4-SGSN LDN" O Ldn 0 [],
        additional_mm_context_for_srvcc: "Additional MM context for SRVCC" CO AdditionalMmContextForSrvcc 0 [],
        additional_flags_for_srvcc: "Additional flags for SRVCC" CO AdditionalFlagsForSrvcc 0 [],
        stn_sr: "STN-SR" CO StnSr 0 [],
        c_msisdn: "C-MSISDN" CO Msisdn 0 [],
        mdt_configuration: "MDT Configuration" CO MdtConfiguration 0 [],
        sgsn_node_name: "SGSN node name" CO Fqdn 1 [],
        mme_node_name: "MME node name" CO Fqdn 2 [],
        user_csg_information: "User CSG Information (UCI)" CO Uci 0 [],
        monitoring_event_information: "Monitoring Event Information" CO* MonitoringEventInformation 0 [],
        monitoring_event_extension_information: "Monitoring Event Extension Information" CO* MonitoringEventExtensionInformation 0 [],
        ue_usage_type: "UE Usage Type" CO IntegerNumber 0 [],
        mme_sgsn_ue_scef_pdn_connections: "MME/SGSN UE SCEF PDN Connections" CO* MmeUeScefPdnConnectionsInForwardRelocationRequest 0 [],
        msisdn: "MSISDN" CO Msisdn 1 [],
        source_udp_port_number: "Source UDP Port Number" CO PortNumber 0 [],
        serving_plmn_rate_control: "Serving PLMN Rate Control" CO ServingPlmnRateControl 0 [],
        extended_trace_information: "Extended Trace Information" C ExtendedTraceInformation 0 [],
        subscribed_additional_rrm_policy_index: "Subscribed Additional RRM Policy Index" CO AdditionalRrmPolicyIndex 0 [],
        additional_rrm_policy_index_in_use: "Additional RRM Policy Index in Use" CO AdditionalRrmPolicyIndex 1 [],
        subscribed_v2x_information: "Subscribed V2X Information" CO SubscribedV2xInformationInForwardRelocationRequest 0 [],
        iwk_scef_id_for_monitoring_event: "IWK SCEF ID for Monitoring Event" CO NodeIdentifier 0 [],
        alternative_imsi: "Alternative IMSI" CO AlternativeImsi 0 [],
        mdt_configuration_nr: "MDT Configuration NR" CO MdtConfigurationNr 0 [],
        trace_collection_entity_uri: "Trace Collection Entity URI" O Uri 0 [],
        disaster_plmn: "Disaster PLMN" C PlmnId 1 [],
    }
}

gtpv2c_table! {
    /// MME/SGSN/AMF UE EPS PDN Connections within Forward Relocation Request (Table 7.3.1-2).
    group MmeSgsnAmfUeEpsPdnConnectionsInForwardRelocationRequest = 109, "PDN Connection", "7.3.1-2" {
        apn: "APN" M Apn 0 [],
        apn_restriction: "APN Restriction" C ApnRestriction 0 [],
        selection_mode: "Selection Mode" CO SelectionMode 0 [],
        ipv4_address: "IPv4 Address" C IpAddress 0 [],
        ipv6_address: "IPv6 Address" C IpAddress 1 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID" M Ebi 0 [],
        pgw_s5_s8_ip_address_for_control_plane_or_pmip: "PGW S5/S8 IP Address for Control Plane or PMIP" M FTeid 0 [],
        pgw_node_name: "PGW node name" CO Fqdn 0 [],
        bearer_contexts: "Bearer Contexts" C* BearerContextInMmeSgsnAmfUeEpsPdnConnectionsInForwardRelocationRequest 0 [],
        aggregate_maximum_bit_rate: "Aggregate Maximum Bit Rate (APN-AMBR)" M Ambr 0 [],
        charging_characteristics: "Charging characteristics" C ChargingCharacteristics 0 [],
        change_reporting_action: "Change Reporting Action" C ChangeReportingAction 0 [],
        csg_information_reporting_action: "CSG Information Reporting Action" CO CsgInformationReportingAction 0 [],
        hnb_information_reporting: "H(e)NB Information Reporting" CO HENbInformationReporting 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        signalling_priority_indication: "Signalling Priority Indication" CO SignallingPriorityIndication 0 [],
        change_to_report_flags: "Change to Report Flags" CO ChangeToReportFlags 0 [],
        local_home_network_id: "Local Home Network ID" CO Fqdn 1 [],
        presence_reporting_area_action: "Presence Reporting Area Action" CO* PresenceReportingAreaAction 0 [],
        wlan_offloadability_indication: "WLAN Offloadability Indication" CO WlanOffloadabilityIndication 0 [],
        remote_ue_context_connected: "Remote UE Context Connected" CO* RemoteUeContextConnectedInMmeSgsnUeEpsPdnConnectionsInForwardRelocationRequest 0 [],
        pdn_type: "PDN Type" CO PdnType 0 [],
        header_compression_configuration: "Header Compression Configuration" CO HeaderCompressionConfiguration 0 [],
        pgw_change_info: "PGW Change Info" CO PgwChangeInfoInForwardRelocationRequest 0 [],
        new_pgw_c_smf_ip_address: "New PGW-C/SMF IP Address" CO IpAddress 2 [],
        new_pgw_node_name: "New PGW Node Name" CO Fqdn 2 [],
        up_security_policy: "UP Security Policy" CO UpSecurityPolicy 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within MME/SGSN/AMF UE EPS PDN Connections within Forward Relocation Request (Table 7.3.1-3).
    group BearerContextInMmeSgsnAmfUeEpsPdnConnectionsInForwardRelocationRequest = 93, "Bearer Context", "7.3.1-3" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        tft: "TFT" C EpsBearerLevelTrafficFlowTemplate 0 [],
        sgw_s1_s4_s12_ip_address_and_teid_for_user_plane: "SGW S1/S4/S12 IP Address and TEID for user plane" M FTeid 0 [],
        pgw_s5_s8_ip_address_and_teid_for_user_plane: "PGW S5/S8 IP Address and TEID for user plane" CO FTeid 1 [],
        bearer_level_qos: "Bearer Level QoS" M BearerLevelQualityOfService 0 [],
        bss_container: "BSS Container" CO FContainer 0 [],
        transaction_identifier: "Transaction Identifier" C Ti 0 [],
        bearer_flags: "Bearer Flags" CO BearerFlags 0 [],
        sgw_s11_ip_address_and_teid_for_user_plane: "SGW S11 IP Address and TEID for user plane" CO FTeid 2 [],
    }
}

gtpv2c_table! {
    /// Remote UE Context Connected within MME/SGSN UE EPS PDN Connections within Forward Relocation Request (Table 7.3.1-4).
    group RemoteUeContextConnectedInMmeSgsnUeEpsPdnConnectionsInForwardRelocationRequest = 191, "Remote UE Context", "7.3.1-4" {
        remote_user_id: "Remote User ID" M RemoteUserId 0 [],
        remote_ue_ip_information: "Remote UE IP Information" M RemoteUeIpInformation 0 [],
    }
}

gtpv2c_table! {
    /// MME UE SCEF PDN Connections within Forward Relocation Request (Table 7.3.1-5).
    group MmeUeScefPdnConnectionsInForwardRelocationRequest = 195, "SCEF PDN Connection", "7.3.1-5" {
        apn: "APN" M Apn 0 [],
        default_eps_bearer_id: "Default EPS Bearer ID" M Ebi 0 [],
        scef_id: "SCEF ID" M NodeIdentifier 0 [],
    }
}

gtpv2c_table! {
    /// Subscribed V2X Information within Forward Relocation Request (Table 7.3.1-6).
    group SubscribedV2xInformationInForwardRelocationRequest = 208, "V2X Context", "7.3.1-6" {
        lte_v2x_services_authorized: "LTE V2X Services Authorized" C ServicesAuthorized 0 [],
        nr_v2x_services_authorized: "NR V2X Services Authorized" C ServicesAuthorized 1 [],
        lte_ue_sidelink_aggregate_maximum_bit_rate: "LTE UE Sidelink Aggregate Maximum Bit Rate" C BitRate 0 [],
        nr_ue_sidelink_aggregate_maximum_bit_rate: "NR UE Sidelink Aggregate Maximum Bit Rate" C BitRate 1 [],
        pc5_qos_parameters: "PC5 QoS Parameters" C Pc5QosParametersInForwardRelocationRequest 0 [],
    }
}

gtpv2c_table! {
    /// PC5 QoS Parameters within Forward Relocation Request (Table 7.3.1-7).
    group Pc5QosParametersInForwardRelocationRequest = 209, "PC5 QoS Parameters", "7.3.1-7" {
        pc5_qos_flows: "PC5 QoS Flows" M* Pc5QosFlow 0 [],
        pc5_link_aggregated_bit_rates: "PC5 Link Aggregated Bit Rates" O BitRate 0 [],
    }
}

gtpv2c_table! {
    /// PGW Change Info with Forward Relocation Request (Table 7.3.1-8).
    group PgwChangeInfoInForwardRelocationRequest = 214, "PGW Change Info", "7.3.1-8" {
        pgw_set_fqdn: "PGW Set FQDN" CO PgwFqdn 0 [],
        alternative_pgw_c_smf_ip_address: "Alternative PGW-C/SMF IP Address" CO* IpAddress 0 [],
        alternative_pgw_c_smf_fqdn: "Alternative PGW-C/SMF FQDN" CO* PgwFqdn 1 [],
        group_id: "Group Id" CO GroupId 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Forward Relocation Response (Table 7.3.2-1).
    message ForwardRelocationResponse = 134, "Forward Relocation Response", "7.3.2-1" {
        cause: "Cause" M Cause 0 [],
        sender_s_f_teid_for_control_plane: "Sender's F-TEID for Control Plane" C FTeid 0 [],
        indication_flags: "Indication Flags" C Indication 0 [],
        list_of_set_up_bearers: "List of Set-up Bearers" C* BearerContextT7_3_2_2 0 [],
        list_of_set_up_rabs: "List of Set-up RABs" C* BearerContextT7_3_2_2 1 [],
        list_of_set_up_pfcs: "List of Set-up PFCs" O* BearerContextT7_3_2_2 2 [],
        s1_ap_cause: "S1-AP Cause" C FCause 0 [],
        ranap_cause: "RANAP Cause" C FCause 1 [],
        bssgp_cause: "BSSGP Cause" C FCause 2 [],
        e_utran_transparent_container: "E-UTRAN Transparent Container" C FContainer 0 [],
        utran_transparent_container: "UTRAN Transparent Container" C FContainer 1 [],
        bss_container: "BSS Container" C FContainer 2 [],
        mme_s4_sgsn_ldn: "MME/S4-SGSN LDN" O Ldn 0 [],
        sgsn_node_name: "SGSN node name" CO Fqdn 0 [],
        mme_node_name: "MME node name" CO Fqdn 1 [],
        sgsn_number: "SGSN Number" O NodeNumber 0 [],
        sgsn_identifier: "SGSN Identifier" O NodeIdentifier 0 [],
        mme_identifier: "MME Identifier" O NodeIdentifier 1 [],
        mme_number_for_mt_sms: "MME number for MT SMS" CO NodeNumber 1 [],
        sgsn_identifier_for_mt_sms: "SGSN Identifier for MT-SMS" CO NodeIdentifier 2 [],
        mme_identifier_for_mt_sms: "MME Identifier for MT SMS" CO NodeIdentifier 3 [],
        list_of_set_up_bearers_for_scef_pdn_connections: "List of Set-up Bearers for SCEF PDN Connections" CO* BearerContextT7_3_2_2 3 [],
        srvcc_rejected_cause: "(v)SRVCC rejected Cause" CO SrvccCause 0 [],
        msc_number: "MSC Number" O NodeNumber 2 [],
        amf_identifier: "AMF Identifier" O NfInstanceId 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context (Table 7.3.2-2).
    group BearerContextT7_3_2_2 = 93, "Bearer Context", "7.3.2-2" {
        eps_bearer_id: "EPS Bearer ID" C Ebi 0 [],
        packet_flow_id: "Packet Flow ID" C PacketFlowId 0 [],
        enodeb_gnodeb_f_teid_for_dl_data_forwarding: "eNodeB/gNodeB F-TEID for DL data forwarding" C FTeid 0 [],
        enodeb_f_teid_for_ul_data_forwarding: "eNodeB F-TEID for UL data forwarding" O FTeid 1 [],
        sgw_upf_f_teid_for_dl_data_forwarding: "SGW/UPF F-TEID for DL data forwarding" CO FTeid 2 [],
        rnc_f_teid_for_dl_data_forwarding: "RNC F-TEID for DL data forwarding" C FTeid 3 [],
        sgsn_f_teid_for_dl_data_forwarding: "SGSN F-TEID for DL data forwarding" C FTeid 4 [],
        sgw_f_teid_for_ul_data_forwarding: "SGW F-TEID for UL data forwarding" O FTeid 5 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Forward Relocation Complete Notification (Table 7.3.3-1).
    message ForwardRelocationCompleteNotification = 135, "Forward Relocation Complete Notification", "7.3.3-1" {
        indication_flags: "Indication Flags" C Indication 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Forward Relocation Complete Acknowledge (Table 7.3.4-1).
    message ForwardRelocationCompleteAcknowledge = 136, "Forward Relocation Complete Acknowledge", "7.3.4-1" {
        cause: "Cause" M Cause 0 [],
        recovery: "Recovery" O Recovery 0 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" CO* SecondaryRatUsageDataReport 0 [],
        secondary_rat_usage_data_report_from_ng_ran: "Secondary RAT Usage Data Report from NG-RAN" CO* SecondaryRatUsageDataReport 1 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Context Request (Table 7.3.5-1).
    message ContextRequest = 130, "Context Request", "7.3.5-1" {
        imsi: "IMSI" C Imsi 0 [],
        guti: "GUTI" CO Guti 0 [],
        routeing_area_identity: "Routeing Area Identity(RAI)" CO Uli 0 [],
        packet_tmsi: "Packet TMSI(P-TMSI)" CO PTmsi 0 [],
        p_tmsi_signature: "P-TMSI Signature" CO PTmsiSignature 0 [],
        complete_tau_request_message: "Complete TAU request message" C CompleteRequestMessage 0 [],
        s3_s16_s10_n26_address_and_teid_for_control_plane: "S3/S16/S10/N26 Address and TEID for Control Plane" C FTeid 0 [],
        udp_source_port_number: "UDP Source Port Number" C PortNumber 0 [],
        rat_type: "RAT Type" C RatType 0 [],
        indication: "Indication" CO Indication 0 [],
        hop_counter: "Hop Counter" O HopCounter 0 [],
        target_plmn_id: "Target PLMN ID" CO ServingNetwork 0 [],
        mme_s4_sgsn_ldn: "MME/S4-SGSN LDN" O Ldn 0 [],
        sgsn_node_name: "SGSN node name" CO Fqdn 0 [],
        mme_node_name: "MME node name" CO Fqdn 1 [],
        sgsn_number: "SGSN Number" O NodeNumber 0 [],
        sgsn_identifier: "SGSN Identifier" O NodeIdentifier 0 [],
        mme_identifier: "MME Identifier" O NodeIdentifier 1 [],
        ciot_optimizations_support_indication: "CIoT Optimizations Support Indication" CO CiotOptimizationsSupportIndication 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Context Response (Table 7.3.6-1).
    message ContextResponse = 131, "Context Response", "7.3.6-1" {
        cause: "Cause" M Cause 0 [],
        imsi: "IMSI" C Imsi 0 [],
        mme_sgsn_amf_ue_mm_context: "MME/SGSN/AMF UE MM Context" C MmContextGsmKeyAndTriplets 0 [],
        mme_sgsn_amf_ue_eps_pdn_connections: "MME/SGSN/AMF UE EPS PDN Connections" C* MmeSgsnAmfUeEpsPdnConnectionsInContextResponse 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" C FTeid 0 [],
        sgw_s11_s4_ip_address_and_teid_for_control_plane: "SGW S11/S4 IP Address and TEID for Control Plane" C FTeid 1 [],
        sgw_node_name: "SGW node name" C Fqdn 0 [],
        indication_flags: "Indication Flags" C Indication 0 [],
        trace_information: "Trace Information" C TraceInformation 0 [],
        hrpd_access_node_s101_ip_address: "HRPD access node S101 IP address" C IpAddress 0 [],
        n1xiws_s102_ip_address: "1xIWS S102 IP address" C IpAddress 1 [],
        subscribed_rfsp_index: "Subscribed RFSP Index" CO RfspIndex 0 [],
        rfsp_index_in_use: "RFSP Index in Use" CO RfspIndex 1 [],
        rfsp_index_in_use_validity_time: "RFSP Index in Use Validity Time" O TimerInSeconds 0 [],
        ue_time_zone: "UE Time Zone" CO UeTimeZone 0 [],
        mme_s4_sgsn_ldn: "MME/S4-SGSN LDN" O Ldn 0 [],
        mdt_configuration: "MDT Configuration" CO MdtConfiguration 0 [],
        sgsn_node_name: "SGSN node name" CO Fqdn 1 [],
        mme_node_name: "MME node name" CO Fqdn 2 [],
        user_csg_information: "User CSG Information (UCI)" CO Uci 0 [],
        monitoring_event_information: "Monitoring Event Information" CO* MonitoringEventInformation 0 [],
        monitoring_event_extension_information: "Monitoring Event Extension Information" CO* MonitoringEventExtensionInformation 0 [],
        ue_usage_type: "UE Usage Type" CO IntegerNumber 0 [],
        mme_sgsn_ue_scef_pdn_connections: "MME/SGSN UE SCEF PDN Connections" C* MmeSgsnUeScefPdnConnectionsInContextResponse 0 [],
        rat_type: "RAT Type" CO RatType 0 [],
        serving_plmn_rate_control: "Serving PLMN Rate Control" CO ServingPlmnRateControl 0 [],
        mo_exception_data_counter: "MO Exception Data Counter" CO Counter 0 [],
        remaining_running_service_gap_timer: "Remaining Running Service Gap Timer" CO IntegerNumber 1 [],
        extended_trace_information: "Extended Trace Information" C ExtendedTraceInformation 0 [],
        subscribed_additional_rrm_policy_index: "Subscribed Additional RRM Policy Index" CO AdditionalRrmPolicyIndex 0 [],
        additional_rrm_policy_index_in_use: "Additional RRM Policy Index in Use" CO AdditionalRrmPolicyIndex 1 [],
        iwk_scef_id_for_monitoring_event: "IWK SCEF ID for Monitoring Event" CO NodeIdentifier 0 [],
        alternative_imsi: "Alternative IMSI" CO AlternativeImsi 0 [],
        msisdn: "MSISDN" CO Msisdn 0 [],
        mdt_configuration_nr: "MDT Configuration NR" CO MdtConfigurationNr 0 [],
        trace_collection_entity_uri: "Trace Collection Entity URI" O Uri 0 [],
        disaster_plmn: "Disaster PLMN" C PlmnId 0 [],
    }
}

gtpv2c_table! {
    /// MME/SGSN/AMF UE EPS PDN Connections within Context Response (Table 7.3.6-2).
    group MmeSgsnAmfUeEpsPdnConnectionsInContextResponse = 109, "PDN Connection", "7.3.6-2" {
        apn: "APN" M Apn 0 [],
        apn_restriction: "APN Restriction" C ApnRestriction 0 [],
        selection_mode: "Selection Mode" CO SelectionMode 0 [],
        ipv4_address: "IPv4 Address" C IpAddress 0 [],
        ipv6_address: "IPv6 Address" C IpAddress 1 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID" M Ebi 0 [],
        pgw_s5_s8_ip_address_for_control_plane_or_pmip: "PGW S5/S8 IP Address for Control Plane or PMIP" M FTeid 0 [],
        pgw_node_name: "PGW node name" CO Fqdn 0 [],
        bearer_contexts: "Bearer Contexts" M* BearerContextInMmeSgsnAmfUeEpsPdnConnectionsInContextResponse 0 [],
        aggregate_maximum_bit_rate: "Aggregate Maximum Bit Rate (APN-AMBR)" M Ambr 0 [],
        charging_characteristics: "Charging characteristics" C ChargingCharacteristics 0 [],
        change_reporting_action: "Change Reporting Action" C ChangeReportingAction 0 [],
        csg_information_reporting_action: "CSG Information Reporting Action" CO CsgInformationReportingAction 0 [],
        hnb_information_reporting: "H(e)NB Information Reporting" CO HENbInformationReporting 0 [],
        indication_flags: "Indication flags" CO Indication 0 [],
        signalling_priority_indication: "Signalling Priority Indication" CO SignallingPriorityIndication 0 [],
        change_to_report_flags: "Change to Report Flags" CO ChangeToReportFlags 0 [],
        local_home_network_id: "Local Home Network ID" CO Fqdn 1 [],
        presence_reporting_area_action: "Presence Reporting Area Action" CO* PresenceReportingAreaAction 0 [],
        wlan_offloadability_indication: "WLAN Offloadability Indication" CO WlanOffloadabilityIndication 0 [],
        remote_ue_context_connected: "Remote UE Context Connected" CO* RemoteUeContextConnectedInMmeSgsnUeEpsPdnConnectionsInContextResponse 0 [],
        pdn_type: "PDN Type" CO PdnType 0 [],
        header_compression_configuration: "Header Compression Configuration" CO HeaderCompressionConfiguration 0 [],
        pgw_change_info: "PGW Change Info" CO PgwChangeInfoInContextResponse 0 [],
        new_pgw_c_smf_ip_address: "New PGW-C/SMF IP Address" CO IpAddress 2 [],
        new_pgw_node_name: "New PGW Node Name" CO Fqdn 2 [],
        up_security_policy: "UP Security Policy" CO UpSecurityPolicy 0 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within MME/SGSN/AMF UE EPS PDN Connections within Context Response (Table 7.3.6-3).
    group BearerContextInMmeSgsnAmfUeEpsPdnConnectionsInContextResponse = 93, "Bearer Context", "7.3.6-3" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        tft: "TFT" C EpsBearerLevelTrafficFlowTemplate 0 [],
        sgw_s1_s4_s12_s11_ip_address_and_teid_for_user_plane: "SGW S1/S4/S12/S11 IP Address and TEID for user plane" C FTeid 0 [],
        pgw_s5_s8_ip_address_and_teid_for_user_plane: "PGW S5/S8 IP Address and TEID for user plane" CO FTeid 1 [],
        bearer_level_qos: "Bearer Level QoS" M BearerLevelQualityOfService 0 [],
        bss_container: "BSS Container" CO FContainer 0 [],
        transaction_identifier: "Transaction Identifier" C Ti 0 [],
        sgw_s11_ip_address_and_teid_for_user_plane: "SGW S11 IP Address and TEID for user plane" CO FTeid 2 [],
    }
}

gtpv2c_table! {
    /// Remote UE Context Connected within MME/SGSN UE EPS PDN Connections within Context Response (Table 7.3.6-4).
    group RemoteUeContextConnectedInMmeSgsnUeEpsPdnConnectionsInContextResponse = 191, "Remote UE Context", "7.3.6-4" {
        remote_user_id: "Remote User ID" M RemoteUserId 0 [],
        remote_ue_ip_information: "Remote UE IP Information" M RemoteUeIpInformation 0 [],
    }
}

gtpv2c_table! {
    /// MME/SGSN UE SCEF PDN Connections within Context Response (Table 7.3.6-5).
    group MmeSgsnUeScefPdnConnectionsInContextResponse = 195, "SCEF PDN Connection", "7.3.6-5" {
        apn: "APN" M Apn 0 [],
        default_eps_bearer_id: "Default EPS Bearer ID" M Ebi 0 [],
        scef_id: "SCEF ID" M NodeIdentifier 0 [],
    }
}

gtpv2c_table! {
    /// PGW Change Info with Context Response (Table 7.3.6-6).
    group PgwChangeInfoInContextResponse = 214, "PGW Change Info", "7.3.6-6" {
        pgw_set_fqdn: "PGW Set FQDN" CO PgwFqdn 0 [],
        alternative_pgw_c_smf_ip_address: "Alternative PGW-C/SMF IP Address" CO* IpAddress 0 [],
        alternative_pgw_c_smf_fqdn: "Alternative PGW-C/SMF FQDN" CO* PgwFqdn 1 [],
        group_id: "Group Id" CO GroupId 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Context Acknowledge (Table 7.3.7-1).
    message ContextAcknowledge = 132, "Context Acknowledge", "7.3.7-1" {
        cause: "Cause" M Cause 0 [],
        indication_flags: "Indication flags" C Indication 0 [],
        forwarding_f_teid: "Forwarding F-TEID" CO FTeid 0 [],
        bearer_contexts: "Bearer Contexts" CO* BearerContextInContextAcknowledge 0 [],
        sgsn_number: "SGSN Number" CO NodeNumber 0 [],
        mme_number_for_mt_sms: "MME number for MT SMS" CO NodeNumber 1 [],
        sgsn_identifier_for_mt_sms: "SGSN Identifier for MT SMS" CO NodeIdentifier 0 [],
        mme_identifier_for_mt_sms: "MME Identifier for MT SMS" CO NodeIdentifier 1 [],
    }
}

gtpv2c_table! {
    /// Bearer Context within Context Acknowledge (Table 7.3.7-2).
    group BearerContextInContextAcknowledge = 93, "Bearer Context", "7.3.7-2" {
        eps_bearer_id: "EPS Bearer ID" M Ebi 0 [],
        forwarding_f_teid: "Forwarding F-TEID" M FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in an Identification Request (Table 7.3.8-1).
    message IdentificationRequest = 128, "Identification Request", "7.3.8-1" {
        guti: "GUTI" C Guti 0 [],
        routeing_area_identity: "Routeing Area Identity(RAI)" C Uli 0 [],
        packet_tmsi: "Packet TMSI(P-TMSI)" C PTmsi 0 [],
        p_tmsi_signature: "P-TMSI Signature" C PTmsiSignature 0 [],
        complete_attach_request_message: "Complete Attach Request Message" C CompleteRequestMessage 0 [],
        address_for_control_plane: "Address for Control Plane" O IpAddress 0 [],
        udp_source_port_number: "UDP Source Port Number" C PortNumber 0 [],
        hop_counter: "Hop Counter" O HopCounter 0 [],
        target_plmn_id: "Target PLMN ID" CO ServingNetwork 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in an Identification Response (Table 7.3.9-1).
    message IdentificationResponse = 129, "Identification Response", "7.3.9-1" {
        cause: "Cause" M Cause 0 [],
        imsi: "IMSI" C Imsi 0 [],
        mme_sgsn_ue_mm_context: "MME/SGSN UE MM Context" C MmContextGsmKeyAndTriplets 0 [],
        trace_information: "Trace Information" CO TraceInformation 0 [],
        ue_usage_type: "UE Usage Type" CO IntegerNumber 0 [],
        monitoring_event_information: "Monitoring Event Information" CO* MonitoringEventInformation 0 [],
        monitoring_event_extension_information: "Monitoring Event Extension Information" CO* MonitoringEventExtensionInformation 0 [],
        extended_trace_information: "Extended Trace Information" C ExtendedTraceInformation 0 [],
        trace_collection_entity_uri: "Trace Collection Entity URI" O Uri 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Forward Access Context Notification (Table 7.3.10-1).
    message ForwardAccessContextNotification = 137, "Forward Access Context Notification", "7.3.10-1" {
        rab_contexts: "RAB Contexts" C* RabContext 0 [],
        source_rnc_pdcp_context_info: "Source RNC PDCP context Info" C SourceRncPdcpContextInfo 0 [],
        pdu_numbers: "PDU Numbers" C PduNumbers 0 [],
        e_utran_transparent_container: "E-UTRAN Transparent Container" C FContainer 0 [],
        e_utran_transparent_container_1: "E-UTRAN Transparent Container" C FContainer 1 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Forward Access Context Acknowledge (Table 7.3.11-1).
    message ForwardAccessContextAcknowledge = 138, "Forward Access Context Acknowledge", "7.3.11-1" {
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Detach Notification (Table 7.3.12-1).
    message DetachNotification = 149, "Detach Notification", "7.3.12-1" {
        cause: "Cause" M Cause 0 [],
        detach_type: "Detach Type" CO DetachType 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Detach Acknowledge (Table 7.3.13-1).
    message DetachAcknowledge = 150, "Detach Acknowledge", "7.3.13-1" {
        cause: "Cause" M Cause 0 [],
        recovery: "Recovery" O Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Change Notification Request (Table 7.3.14-1).
    message ChangeNotificationRequest = 38, "Change Notification Request", "7.3.14-1" {
        imsi: "IMSI" C Imsi 0 [],
        me_identity: "ME Identity (MEI)" C Mei 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        rat_type: "RAT Type" M RatType 0 [],
        user_location_information: "User Location Information (ULI)" CO Uli 0 [],
        user_csg_information: "User CSG Information (UCI)" CO Uci 0 [],
        pgw_s5_s8_gtp_c_ip_address: "PGW S5/S8 GTP-C IP Address" CO IpAddress 0 [],
        lbi: "LBI" CO Ebi 0 [],
        presence_reporting_area_information: "Presence Reporting Area Information" CO* PresenceReportingAreaInformation 0 [],
        mo_exception_data_counter: "MO Exception Data Counter" CO Counter 0 [],
        secondary_rat_usage_data_report: "Secondary RAT Usage Data Report" O* SecondaryRatUsageDataReport 0 [],
        pscell_id: "PSCell ID" CO PscellId 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Change Notification Response (Table 7.3.15-1).
    message ChangeNotificationResponse = 39, "Change Notification Response", "7.3.15-1" {
        imsi: "IMSI" C Imsi 0 [],
        me_identity: "ME Identity (MEI)" C Mei 0 [],
        cause: "Cause" M Cause 0 [],
        change_reporting_action: "Change Reporting Action" C ChangeReportingAction 0 [],
        csg_information_reporting_action: "CSG Information Reporting Action" CO CsgInformationReportingAction 0 [],
        presence_reporting_area_action: "Presence Reporting Area Action" CO* PresenceReportingAreaAction 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in Relocation Cancel Request (Table 7.3.16-1).
    message RelocationCancelRequest = 139, "Relocation Cancel Request", "7.3.16-1" {
        imsi: "IMSI" C Imsi 0 [],
        me_identity: "ME Identity (MEI)" C Mei 0 [],
        indication_flags: "Indication Flags" CO Indication 0 [],
        ranap_cause: "RANAP Cause" C FCause 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in Relocation Cancel Response (Table 7.3.17-1).
    message RelocationCancelResponse = 140, "Relocation Cancel Response", "7.3.17-1" {
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Configuration Transfer Tunnel Message (Table 7.3.18-1).
    message ConfigurationTransferTunnelMessage = 141, "Configuration Transfer Tunnel Message", "7.3.18-1" {
        e_utran_transparent_container_en_dc_container_inter_system_son_container: "E-UTRAN Transparent Container / EN-DC Container / Inter-system SON Container" M FContainer 0 [],
        target_enodeb_id_en_gnb_id_gnb_id: "Target eNodeB ID / en-gNB ID / gnB ID" M TargetIdentification 0 [],
        connected_target_enodeb_id: "Connected Target eNodeB ID" CO TargetIdentification 1 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a RAN Information Relay (Table 7.3.19-1).
    message RanInformationRelay = 152, "RAN Information Relay", "7.3.19-1" {
        bss_container: "BSS Container" M FContainer 0 [],
        rim_routing_address: "RIM Routing Address" C TargetIdentification 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in an ISR Status Indication (Table 7.3.20-1).
    message IsrStatusIndication = 157, "ISR Status Indication", "7.3.20-1" {
        action_indication: "Action Indication" M ActionIndication 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in UE Registration Query Request (Table 7.3.21-1).
    message UeRegistrationQueryRequest = 158, "UE Registration Query Request", "7.3.21-1" {
        imsi: "IMSI" M Imsi 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in UE Registration Query Response (Table 7.3.22-1).
    message UeRegistrationQueryResponse = 159, "UE Registration Query Response", "7.3.22-1" {
        cause: "Cause" M Cause 0 [],
        imsi: "IMSI" M Imsi 0 [],
        selected_core_network_operator_identifier: "Selected Core Network Operator Identifier" M PlmnId 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Suspend Notification (Table 7.4.1-1).
    message SuspendNotification = 162, "Suspend Notification", "7.4.1-1" {
        imsi: "IMSI" C Imsi 0 [],
        routeing_area_identity: "Routeing Area Identity(RAI)" CO Uli 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID (LBI)" CO Ebi 0 [],
        packet_tmsi: "Packet TMSI(P-TMSI)" CO PTmsi 0 [],
        originating_node: "Originating Node" CO NodeType 0 [],
        address_for_control_plane: "Address for Control Plane" CO IpAddress 0 [],
        udp_source_port_number: "UDP Source Port Number" CO PortNumber 0 [],
        hop_counter: "Hop Counter" O HopCounter 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" CO FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Suspend Acknowledge (Table 7.4.2-1).
    message SuspendAcknowledge = 163, "Suspend Acknowledge", "7.4.2-1" {
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Resume Notification (Table 7.4.3-1).
    message ResumeNotification = 164, "Resume Notification", "7.4.3-1" {
        imsi: "IMSI" M Imsi 0 [],
        linked_eps_bearer_id: "Linked EPS Bearer ID (LBI)" CO Ebi 0 [],
        originating_node: "Originating Node" CO NodeType 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" CO FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Resume Acknowledge (Table 7.4.4-1).
    message ResumeAcknowledge = 165, "Resume Acknowledge", "7.4.4-1" {
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in CS Paging Indication (Table 7.4.5-1).
    message CsPagingIndication = 151, "CS Paging Indication", "7.4.5-1" {
        imsi: "IMSI" M Imsi 0 [],
        vlr_name: "VLR Name" M Fqdn 0 [],
        tmsi: "TMSI" O Tmsi 0 [],
        location_area_identifier: "Location area identifier" O Uli 0 [],
        global_cn_id: "Global CN-Id" O GlobalCnId 0 [],
        channel_needed: "Channel needed" O ChannelNeeded 0 [],
        emlpp_priority: "eMLPP Priority" O EmlppPriority 0 [],
        service_indicator: "Service Indicator" CO ServiceIndicator 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in Alert MME Notification (Table 7.4.6-1).
    message AlertMmeNotification = 153, "Alert MME Notification", "7.4.6-1" {
    }
}

gtpv2c_table! {
    /// Information Elements in Alert MME Acknowledge (Table 7.4.7-1).
    message AlertMmeAcknowledge = 154, "Alert MME Acknowledge", "7.4.7-1" {
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Element in UE Activity Notification (Table 7.4.8-1).
    message UeActivityNotification = 155, "UE Activity Notification", "7.4.8-1" {
    }
}

gtpv2c_table! {
    /// Information Elements in UE Activity Acknowledge (Table 7.4.9-1).
    message UeActivityAcknowledge = 156, "UE Activity Acknowledge", "7.4.9-1" {
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Create Forwarding Tunnel Request (Table 7.5.1-1).
    message CreateForwardingTunnelRequest = 160, "Create Forwarding Tunnel Request", "7.5.1-1" {
        s103_pdn_data_forwarding_info: "S103 PDN Data Forwarding Info" M* S103pdf 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Create Forwarding Tunnel Response (Table 7.5.2-1).
    message CreateForwardingTunnelResponse = 161, "Create Forwarding Tunnel Response", "7.5.2-1" {
        cause: "Cause" M Cause 0 [],
        s1_u_data_forwarding_info: "S1-U Data Forwarding Info" C* S1udf 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Delete PDN Connection Set Request (Table 7.9.1-1).
    message DeletePdnConnectionSetRequest = 101, "Delete PDN Connection Set Request", "7.9.1-1" {
        mme_fq_csid: "MME-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
        pgw_fq_csid: "PGW-FQ-CSID" C FqCsid 2 [],
        epdg_fq_csid: "ePDG-FQ-CSID" C FqCsid 3 [],
        twan_fq_csid: "TWAN-FQ-CSID" C FqCsid 4 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Delete PDN Connection Set Response (Table 7.9.2-1).
    message DeletePdnConnectionSetResponse = 102, "Delete PDN Connection Set Response", "7.9.2-1" {
        cause: "Cause" M Cause 0 [],
        recovery: "Recovery" CO Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Update PDN Connection Set Request (Table 7.9.3-1).
    message UpdatePdnConnectionSetRequest = 200, "Update PDN Connection Set Request", "7.9.3-1" {
        mme_fq_csid: "MME-FQ-CSID" C FqCsid 0 [],
        sgw_fq_csid: "SGW-FQ-CSID" C FqCsid 1 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Update PDN Connection Set Response (Table 7.9.4-1).
    message UpdatePdnConnectionSetResponse = 201, "Update PDN Connection Set Response", "7.9.4-1" {
        cause: "Cause" M Cause 0 [],
        pgw_fq_csid: "PGW-FQ-CSID" C FqCsid 0 [],
        recovery: "Recovery" CO Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in PGW Restart Notification (Table 7.9.5-1).
    message PgwRestartNotification = 179, "PGW Restart Notification", "7.9.5-1" {
        pgw_s5_s8_ip_address_for_control_plane_or_pmip: "PGW S5/S8 IP Address for Control Plane or PMIP" M IpAddress 0 [],
        sgw_s11_s4_ip_address_for_control_plane: "SGW S11/S4 IP Address for Control Plane" M IpAddress 1 [],
        cause: "Cause" CO Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in PGW Restart Notification Acknowledge (Table 7.9.6-1).
    message PgwRestartNotificationAcknowledge = 180, "PGW Restart Notification Acknowledge", "7.9.6-1" {
        cause: "Cause" M Cause 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in PGW Downlink Triggering Notification (Table 7.9.7-1).
    message PgwDownlinkTriggeringNotification = 103, "PGW Downlink Triggering Notification", "7.9.7-1" {
        imsi: "IMSI" M Imsi 0 [],
        mme_s4_sgsn_identifier: "MME/S4-SGSN identifier" C IpAddress 0 [],
        pgw_s5_f_teid_for_gtp_or_pmip_control_plane: "PGW S5 F-TEID for GTP or PMIP Control Plane" O FTeid 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in PGW Downlink Triggering Acknowledge (Table 7.9.8-1).
    message PgwDownlinkTriggeringAcknowledge = 104, "PGW Downlink Triggering Acknowledge", "7.9.8-1" {
        cause: "Cause" M Cause 0 [],
        imsi: "IMSI" C Imsi 0 [],
        mme_s4_sgsn_identifier: "MME/S4-SGSN identifier" C IpAddress 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Trace Session Activation (Table 7.12.1-1).
    message TraceSessionActivation = 71, "Trace Session Activation", "7.12.1-1" {
        imsi: "IMSI" C Imsi 0 [],
        trace_information: "Trace Information" M TraceInformation 0 [],
        me_identity: "ME Identity (MEI)" CO Mei 0 [],
        trace_collection_entity_uri: "Trace Collection Entity URI" O Uri 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a Trace Session Deactivation (Table 7.12.2-1).
    message TraceSessionDeactivation = 72, "Trace Session Deactivation", "7.12.2-1" {
        trace_reference: "Trace Reference" M TraceReference 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a MBMS Session Start Request (Table 7.13.1-1).
    message MbmsSessionStartRequest = 231, "MBMS Session Start Request", "7.13.1-1" {
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" M FTeid 0 [],
        temporary_mobile_group_identity: "Temporary Mobile Group Identity (TMGI)" M Tmgi 0 [],
        mbms_session_duration: "MBMS Session Duration" M MbmsSessionDuration 0 [],
        mbms_service_area: "MBMS Service Area" M MbmsServiceArea 0 [],
        mbms_session_identifier: "MBMS Session Identifier" C MbmsSessionIdentifier 0 [],
        mbms_flow_identifier: "MBMS Flow Identifier" C MbmsFlowIdentifier 0 [],
        qos_profile: "QoS profile" M BearerLevelQualityOfService 0 [],
        mbms_ip_multicast_distribution: "MBMS IP Multicast Distribution" M MbmsIpMulticastDistribution 0 [],
        recovery: "Recovery" C Recovery 0 [],
        mbms_time_to_data_transfer: "MBMS Time to Data Transfer" CO MbmsTimeToDataTransfer 0 [],
        mbms_data_transfer_start: "MBMS Data Transfer Start" CO AbsoluteTimeOfMbmsDataTransfer 0 [],
        mbms_flags: "MBMS Flags" CO MbmsFlags 0 [],
        mbms_alternative_ip_multicast_distribution: "MBMS Alternative IP Multicast Distribution" CO MbmsIpMulticastDistribution 1 [],
        mbms_cell_list: "MBMS Cell List" CO EcgiList 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a MBMS Session Start Response (Table 7.13.2-1).
    message MbmsSessionStartResponse = 232, "MBMS Session Start Response", "7.13.2-1" {
        cause: "Cause" M Cause 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" M FTeid 0 [],
        mbms_distribution_acknowledge: "MBMS Distribution Acknowledge" C MbmsDistributionAcknowledge 0 [],
        sn_u_sgsn_f_teid: "Sn-U SGSN F-TEID" C FTeid 1 [],
        recovery: "Recovery" C Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a MBMS Session Update Request (Table 7.13.3-1).
    message MbmsSessionUpdateRequest = 233, "MBMS Session Update Request", "7.13.3-1" {
        mbms_service_area: "MBMS Service Area" C MbmsServiceArea 0 [],
        temporary_mobile_group_identity: "Temporary Mobile Group Identity (TMGI)" M Tmgi 0 [],
        sender_f_teid_for_control_plane: "Sender F-TEID for Control Plane" O FTeid 0 [],
        mbms_session_duration: "MBMS Session Duration" M MbmsSessionDuration 0 [],
        qos_profile: "QoS profile" M BearerLevelQualityOfService 0 [],
        mbms_session_identifier: "MBMS Session Identifier" C MbmsSessionIdentifier 0 [],
        mbms_flow_identifier: "MBMS Flow Identifier" C MbmsFlowIdentifier 0 [],
        mbms_time_to_data_transfer: "MBMS Time to Data Transfer" CO MbmsTimeToDataTransfer 0 [],
        mbms_data_transfer_start_update_stop: "MBMS Data Transfer Start / Update / Stop" CO AbsoluteTimeOfMbmsDataTransfer 0 [],
        mbms_cell_list: "MBMS Cell List" CO EcgiList 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a MBMS Session Update Response (Table 7.13.4-1).
    message MbmsSessionUpdateResponse = 234, "MBMS Session Update Response", "7.13.4-1" {
        cause: "Cause" M Cause 0 [],
        mbms_distribution_acknowledge: "MBMS Distribution Acknowledge" C MbmsDistributionAcknowledge 0 [],
        sn_u_sgsn_f_teid: "Sn-U SGSN F-TEID" C FTeid 0 [],
        recovery: "Recovery" C Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a MBMS Session Stop Request (Table 7.13.5-1).
    message MbmsSessionStopRequest = 235, "MBMS Session Stop Request", "7.13.5-1" {
        mbms_flow_identifier: "MBMS Flow Identifier" C MbmsFlowIdentifier 0 [],
        mbms_data_transfer_stop: "MBMS Data Transfer Stop" CO AbsoluteTimeOfMbmsDataTransfer 0 [],
        mbms_flags: "MBMS Flags" CO MbmsFlags 0 [],
    }
}

gtpv2c_table! {
    /// Information Elements in a MBMS Session Stop Response (Table 7.13.6-1).
    message MbmsSessionStopResponse = 236, "MBMS Session Stop Response", "7.13.6-1" {
        cause: "Cause" M Cause 0 [],
        recovery: "Recovery" CO Recovery 0 [],
    }
}

gtpv2c_table! {
    /// Version Not Supported Indication (§7.1.3): the header alone.
    message VersionNotSupportedIndication = 3, "Version Not Supported Indication", "7.1.3" {
    }
}

