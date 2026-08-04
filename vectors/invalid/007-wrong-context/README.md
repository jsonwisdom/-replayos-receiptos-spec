# Invalid Vector — 007-wrong-context

Expected error: `CONTEXT_BINDING_FAILED`

This schema-valid, correctly signed receipt binds a context digest other than the canonical `RAA-URP-1` context hash.

A conforming verifier MUST reject it with `CONTEXT_BINDING_FAILED`.
