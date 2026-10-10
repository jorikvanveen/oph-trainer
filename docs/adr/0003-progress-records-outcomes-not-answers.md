# Progress records outcomes, not answers

Keep correct and incorrect outcome history by stable question ID, and derive practice metrics from that history. Do not retain answer text, question revisions, or grader identity; a user-corrected result has the same meaning as any other correct result. This keeps accounts limited to the progress the trainer needs, but means the app cannot reconstruct or regrade historical explanations.

Only completed quizzes contribute outcomes. Abandoning a quiz discards its entire batch of results; there is no autosave or resume for unfinished quizzes. This keeps quiz persistence simple, at the cost of not counting practice from a partially completed quiz.
