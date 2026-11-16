/// Whether a message type is an initial message of Table 6.1-1, which its receiver answers:
/// a request, a command or a notification. A type that the table does not have is none.
pub fn is_initial(message_type: u8) -> bool {
    matches!(message_type, 1 | 32 | 34 | 36 | 38 | 40 | 64 | 66 | 68 | 70 | 71 | 72 | 73 | 95 | 97 | 99 | 101 | 103 | 128 | 130 | 133 | 135 | 137 | 139 | 141 | 149 | 151 | 152 | 153 | 155 | 157 | 158 | 160 | 162 | 164 | 166 | 168 | 170 | 176 | 179 | 200 | 211 | 231 | 233 | 235)
}

impl Body {
    /// The Cause of the message at its level, of instance 0.
    pub fn cause(&self) -> Option<CauseValue> {
        let cause = match self {
            Self::CreateSessionResponse(body) => &body.cause,
            Self::CreateBearerResponse(body) => &body.cause,
            Self::BearerResourceFailureIndication(body) => &body.cause,
            Self::ModifyBearerResponse(body) => &body.cause,
            Self::DeleteSessionRequest(body) => &body.cause,
            Self::DeleteBearerRequest(body) => &body.cause,
            Self::DeleteSessionResponse(body) => &body.cause,
            Self::DeleteBearerResponse(body) => &body.cause,
            Self::DownlinkDataNotification(body) => &body.cause,
            Self::DownlinkDataNotificationAcknowledge(body) => &body.cause,
            Self::DownlinkDataNotificationFailureIndication(body) => &body.cause,
            Self::DeleteIndirectDataForwardingTunnelResponse(body) => &body.cause,
            Self::ModifyBearerFailureIndication(body) => &body.cause,
            Self::UpdateBearerResponse(body) => &body.cause,
            Self::DeleteBearerFailureIndication(body) => &body.cause,
            Self::CreateIndirectDataForwardingTunnelResponse(body) => &body.cause,
            Self::ReleaseAccessBearersResponse(body) => &body.cause,
            Self::ModifyAccessBearersResponse(body) => &body.cause,
            Self::RemoteUeReportAcknowledge(body) => &body.cause,
            Self::ForwardRelocationResponse(body) => &body.cause,
            Self::ForwardRelocationCompleteAcknowledge(body) => &body.cause,
            Self::ContextResponse(body) => &body.cause,
            Self::ContextAcknowledge(body) => &body.cause,
            Self::IdentificationResponse(body) => &body.cause,
            Self::ForwardAccessContextAcknowledge(body) => &body.cause,
            Self::DetachNotification(body) => &body.cause,
            Self::DetachAcknowledge(body) => &body.cause,
            Self::ChangeNotificationResponse(body) => &body.cause,
            Self::RelocationCancelResponse(body) => &body.cause,
            Self::UeRegistrationQueryResponse(body) => &body.cause,
            Self::SuspendAcknowledge(body) => &body.cause,
            Self::ResumeAcknowledge(body) => &body.cause,
            Self::AlertMmeAcknowledge(body) => &body.cause,
            Self::UeActivityAcknowledge(body) => &body.cause,
            Self::CreateForwardingTunnelResponse(body) => &body.cause,
            Self::DeletePdnConnectionSetResponse(body) => &body.cause,
            Self::UpdatePdnConnectionSetResponse(body) => &body.cause,
            Self::PgwRestartNotification(body) => &body.cause,
            Self::PgwRestartNotificationAcknowledge(body) => &body.cause,
            Self::PgwDownlinkTriggeringAcknowledge(body) => &body.cause,
            Self::MbmsSessionStartResponse(body) => &body.cause,
            Self::MbmsSessionUpdateResponse(body) => &body.cause,
            Self::MbmsSessionStopResponse(body) => &body.cause,
            _ => return None,
        };
        cause.as_ref().map(|cause| cause.value)
    }
}

impl Body {
    /// The restart counter of the Recovery of the message, of instance 0 (§8.5).
    pub fn recovery(&self) -> Option<u8> {
        let recovery = match self {
            Self::EchoRequest(body) => &body.recovery,
            Self::EchoResponse(body) => &body.recovery,
            Self::CreateSessionRequest(body) => &body.recovery,
            Self::CreateSessionResponse(body) => &body.recovery,
            Self::CreateBearerResponse(body) => &body.recovery,
            Self::BearerResourceFailureIndication(body) => &body.recovery,
            Self::ModifyBearerRequest(body) => &body.recovery,
            Self::ModifyBearerResponse(body) => &body.recovery,
            Self::DeleteSessionResponse(body) => &body.recovery,
            Self::DeleteBearerResponse(body) => &body.recovery,
            Self::DownlinkDataNotificationAcknowledge(body) => &body.recovery,
            Self::DeleteIndirectDataForwardingTunnelResponse(body) => &body.recovery,
            Self::ModifyBearerFailureIndication(body) => &body.recovery,
            Self::UpdateBearerResponse(body) => &body.recovery,
            Self::DeleteBearerFailureIndication(body) => &body.recovery,
            Self::CreateIndirectDataForwardingTunnelRequest(body) => &body.recovery,
            Self::CreateIndirectDataForwardingTunnelResponse(body) => &body.recovery,
            Self::ReleaseAccessBearersResponse(body) => &body.recovery,
            Self::ModifyAccessBearersRequest(body) => &body.recovery,
            Self::ModifyAccessBearersResponse(body) => &body.recovery,
            Self::ForwardRelocationRequest(body) => &body.recovery,
            Self::ForwardRelocationCompleteAcknowledge(body) => &body.recovery,
            Self::DetachAcknowledge(body) => &body.recovery,
            Self::DeletePdnConnectionSetResponse(body) => &body.recovery,
            Self::UpdatePdnConnectionSetResponse(body) => &body.recovery,
            Self::MbmsSessionStartRequest(body) => &body.recovery,
            Self::MbmsSessionStartResponse(body) => &body.recovery,
            Self::MbmsSessionUpdateResponse(body) => &body.recovery,
            Self::MbmsSessionStopResponse(body) => &body.recovery,
            _ => return None,
        };
        recovery.as_ref().map(|recovery| recovery.value)
    }
}

impl Body {
    /// Whether the message lacks its mandatory MM Context, of whichever type (§8.38).
    pub(crate) fn lacks_mm_context(&self) -> bool {
        match self {
            Self::ForwardRelocationRequest(body) => body.mme_sgsn_amf_ue_mm_context_gsm_triplets.is_none() && body.mme_sgsn_amf_ue_mm_context_umts_cipher_quintuplets.is_none() && body.mme_sgsn_amf_ue_mm_context_gsm_cipher_quintuplets.is_none() && body.mme_sgsn_amf_ue_mm_context_umts_quintuplets.is_none() && body.mme_sgsn_amf_ue_mm_context_eps.is_none() && body.mme_sgsn_amf_ue_mm_context_umts_quadruplets_quintuplets.is_none(),
            _ => false,
        }
    }
}

impl crate::registry::CreateSessionResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::CreateBearerResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::BearerResourceFailureIndication {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ModifyBearerResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::DeleteSessionResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::DeleteBearerResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::DownlinkDataNotificationAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::DeleteIndirectDataForwardingTunnelResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ModifyBearerFailureIndication {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::UpdateBearerResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::DeleteBearerFailureIndication {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::CreateIndirectDataForwardingTunnelResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ReleaseAccessBearersResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ModifyAccessBearersResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::RemoteUeReportAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ForwardRelocationResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ForwardRelocationCompleteAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ContextResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ContextAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::IdentificationResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ForwardAccessContextAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::DetachAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ChangeNotificationResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::RelocationCancelResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::UeRegistrationQueryResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::SuspendAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::ResumeAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::AlertMmeAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::UeActivityAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::CreateForwardingTunnelResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::DeletePdnConnectionSetResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::UpdatePdnConnectionSetResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::PgwRestartNotificationAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::PgwDownlinkTriggeringAcknowledge {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::MbmsSessionStartResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::MbmsSessionUpdateResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

impl crate::registry::MbmsSessionStopResponse {
    /// The response of its Cause alone, as a rejection is (§6.1.1): the other IEs that a
    /// rejection may carry are added with their `with_` builders.
    pub fn rejected(cause: CauseValue) -> Self {
        Self { cause: Some(crate::Cause::new(cause)), ..Self::default() }
    }
}

gtpv2c_messages! {
    1 EchoRequest [],
    2 EchoResponse [],
    3 VersionNotSupportedIndication [],
    32 CreateSessionRequest [],
    33 CreateSessionResponse [],
    34 ModifyBearerRequest [],
    35 ModifyBearerResponse [],
    36 DeleteSessionRequest [],
    37 DeleteSessionResponse [],
    38 ChangeNotificationRequest [],
    39 ChangeNotificationResponse [],
    40 RemoteUeReportNotification [],
    41 RemoteUeReportAcknowledge [],
    64 ModifyBearerCommand [],
    65 ModifyBearerFailureIndication [],
    66 DeleteBearerCommand [],
    67 DeleteBearerFailureIndication [],
    68 BearerResourceCommand [],
    69 BearerResourceFailureIndication [],
    70 DownlinkDataNotificationFailureIndication [],
    71 TraceSessionActivation [],
    72 TraceSessionDeactivation [],
    73 StopPagingIndication [],
    95 CreateBearerRequest [],
    96 CreateBearerResponse [],
    97 UpdateBearerRequest [],
    98 UpdateBearerResponse [],
    99 DeleteBearerRequest [],
    100 DeleteBearerResponse [],
    101 DeletePdnConnectionSetRequest [],
    102 DeletePdnConnectionSetResponse [],
    103 PgwDownlinkTriggeringNotification [],
    104 PgwDownlinkTriggeringAcknowledge [],
    128 IdentificationRequest [],
    129 IdentificationResponse [],
    130 ContextRequest [],
    131 ContextResponse [],
    132 ContextAcknowledge [],
    133 ForwardRelocationRequest [],
    134 ForwardRelocationResponse [],
    135 ForwardRelocationCompleteNotification [],
    136 ForwardRelocationCompleteAcknowledge [],
    137 ForwardAccessContextNotification [],
    138 ForwardAccessContextAcknowledge [],
    139 RelocationCancelRequest [],
    140 RelocationCancelResponse [],
    141 ConfigurationTransferTunnelMessage [],
    149 DetachNotification [],
    150 DetachAcknowledge [],
    151 CsPagingIndication [],
    152 RanInformationRelay [],
    153 AlertMmeNotification [],
    154 AlertMmeAcknowledge [],
    155 UeActivityNotification [],
    156 UeActivityAcknowledge [],
    157 IsrStatusIndication [],
    158 UeRegistrationQueryRequest [],
    159 UeRegistrationQueryResponse [],
    160 CreateForwardingTunnelRequest [],
    161 CreateForwardingTunnelResponse [],
    162 SuspendNotification [],
    163 SuspendAcknowledge [],
    164 ResumeNotification [],
    165 ResumeAcknowledge [],
    166 CreateIndirectDataForwardingTunnelRequest [],
    167 CreateIndirectDataForwardingTunnelResponse [],
    168 DeleteIndirectDataForwardingTunnelRequest [],
    169 DeleteIndirectDataForwardingTunnelResponse [],
    170 ReleaseAccessBearersRequest [],
    171 ReleaseAccessBearersResponse [],
    176 DownlinkDataNotification [],
    177 DownlinkDataNotificationAcknowledge [],
    179 PgwRestartNotification [],
    180 PgwRestartNotificationAcknowledge [],
    200 UpdatePdnConnectionSetRequest [],
    201 UpdatePdnConnectionSetResponse [],
    211 ModifyAccessBearersRequest [],
    212 ModifyAccessBearersResponse [],
    231 MbmsSessionStartRequest [],
    232 MbmsSessionStartResponse [],
    233 MbmsSessionUpdateRequest [],
    234 MbmsSessionUpdateResponse [],
    235 MbmsSessionStopRequest [],
    236 MbmsSessionStopResponse [],
}
