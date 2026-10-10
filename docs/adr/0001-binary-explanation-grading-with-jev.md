# Binary explanation grading with Jev

Grade explanation answers with the direct Jev decision model through OpenRouter's Decisions API, rather than a generative chat model. Jev's low cost makes server-funded grading practical for a free public trainer, but it does not generate text, so grades are correct or incorrect without model-written explanations. Short answers use deterministic matching instead, and no other runtime AI integration is planned.

Use the direct Jev model, not Jev Router, which selects other models. See the [Jev introduction](https://typesafe.ai/blog/introducing-system-one-models-and-jev) and [OpenRouter Jev documentation](https://openrouter.ai/docs/guides/community/jev).
