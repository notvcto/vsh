# VSH Governance

**Version 1.0 - February 2026**

VSH is community-driven from day one. This document explains how we make decisions, who has what authority, and how you can influence the project's direction.

---

## Core Principles

1. **Community First** - The community's needs drive decisions
2. **Transparency** - All decisions made in public with clear rationale
3. **Meritocracy** - Influence earned through contribution and judgment
4. **Inclusivity** - All voices heard, regardless of experience level
5. **Sustainability** - Governance that works long-term
6. **Flexibility** - Can adapt as project matures

---

## Governance Structure

```
┌──────────────────────────────────────────────┐
│         Community (All Contributors)         │
│  - Propose features via RFCs                 │
│  - Vote on major decisions                   │
│  - Participate in discussions               │
│  - Shape project direction                   │
└────────────────┬─────────────────────────────┘
                 │
                 │ Elects every 6 months
                 ▼
┌──────────────────────────────────────────────┐
│           Core Team (3-7 members)            │
│  - Day-to-day technical decisions            │
│  - Review and merge PRs                      │
│  - Triage issues                             │
│  - Maintain quality standards                │
│  - Guide project direction                   │
│  - Elected by community vote                 │
└────────────────┬─────────────────────────────┘
                 │
                 │ Includes
                 ▼
┌──────────────────────────────────────────────┐
│            Founder (Victor)                  │
│  - Vision keeper                             │
│  - Tie-breaker on deadlocks                  │
│  - Can veto governance changes               │
│  - Benevolent dictator... for now            │
└──────────────────────────────────────────────┘
```

---

## Decision-Making Framework

### Tier 1: Trivial Decisions (No Vote Needed)

**Examples:**
- Bug fixes
- Typo corrections
- Code cleanup/refactoring
- Test additions
- Documentation improvements (non-breaking)
- Performance optimizations

**Process:**
1. Submit PR
2. Get 1-2 approvals from core team
3. Merge

**Timeline:** 1-3 days

---

### Tier 2: Minor Decisions (Core Team)

**Examples:**
- New optional features (toggleable off)
- Non-breaking enhancements
- New commands (if they follow existing patterns)
- Dependency updates
- CI/CD improvements
- Website updates

**Process:**
1. Create issue or RFC (lightweight)
2. Core team discussion
3. 2/3 majority of core team approves
4. Announce decision in Discord

**Timeline:** 1-2 weeks

---

### Tier 3: Major Decisions (Community Vote)

**Examples:**
- Breaking changes
- New syntax variants
- Removing features
- License changes
- Governance changes
- Core architecture changes
- Default behavior changes

**Process:**
1. Create formal RFC (see RFC Process below)
2. 2-week discussion period
3. Community vote (GitHub poll + Discord)
4. Requires >50% approval + quorum (20% of active contributors)
5. Core team implements or delegates

**Timeline:** 3-4 weeks minimum

---

### Tier 4: Critical Decisions (Founder Veto Power)

**Examples:**
- Changing VSH's core vision
- Changing license
- Transferring project ownership
- Major governance restructuring
- Decisions that could harm community

**Process:**
- Same as Tier 3, but founder can veto
- Founder must explain veto publicly
- Founder veto can be overridden by 75% community vote

**Use:** Should be rare, reserved for protecting project values

---

## Roles & Responsibilities

### Community Members (Everyone!)

**Who:** Anyone who has contributed to VSH in any way

**Responsibilities:**
- Participate in discussions
- Propose ideas and features
- Report bugs
- Help other users
- Vote on major decisions

**Rights:**
- Propose RFCs
- Vote on community decisions
- Nominate core team members
- Run for core team

**How to Join:**
- Make any contribution (code, docs, bug reports, etc.)
- You're automatically a community member!

---

### Core Team Members

**Who:** 3-7 elected contributors

**Responsibilities:**
- Review and merge PRs
- Triage and prioritize issues
- Guide technical direction
- Mentor new contributors
- Represent community interests
- Attend bi-weekly core team meetings
- Respond to questions in Discord
- Make day-to-day decisions

**Requirements:**
- Active contributor for 3+ months
- Demonstrated good judgment
- Collaborative and respectful
- Available 5+ hours/week
- Committed for 6-month term

**Election:**
- Every 6 months (June 1, December 1)
- Nominations: 2 weeks before election
- Self-nominations allowed
- Community votes
- Top 3-7 vote-getters elected
- Term limits: None (but must be re-elected)

**Current Core Team:**
- TBD (First election: June 2026)

---

### Founder (Victor)

**Responsibilities:**
- Keep project aligned with original vision
- Tie-breaker on deadlocked votes
- Veto power on critical decisions (rarely used)
- Long-term strategic planning
- Community stewardship

**Plan:**
- Active in early days (2026-2027)
- Gradually reduce direct involvement
- Transition to advisory role
- Eventually step back if project is healthy

**Succession:**
- If founder steps down, core team becomes final authority
- Can appoint new "vision keeper" by consensus
- Or transition to pure community governance

---

## RFC Process

**RFC = Request for Comments**

Use RFCs for any major decision or significant change.

### When to Write an RFC

**Required:**
- Breaking changes
- New syntax variants
- Major architecture changes
- Removing features
- Changes to defaults
- Governance changes

**Optional but Encouraged:**
- Complex new features
- Performance-critical changes
- Anything controversial
- When you want community input

### RFC Template

Copy `docs/rfcs/0000-template.md`:

```markdown
# RFC-XXXX: [Title]

- **Author:** Your Name
- **Status:** Proposed | Accepted | Rejected | Implemented
- **Created:** YYYY-MM-DD
- **Updated:** YYYY-MM-DD

## Summary
[One paragraph explanation]

## Motivation
[Why are we doing this? What problem does it solve?]

## Detailed Design
[Explain the proposal in detail]

## Examples
[Show how it would work with concrete examples]

## Drawbacks
[Why should we NOT do this?]

## Alternatives
[What other approaches were considered?]

## Unresolved Questions
[What parts are still unclear?]
```

### RFC Lifecycle

```
1. Draft → Author writes RFC
   ↓
2. Proposed → Submit PR to rfcs/ directory
   ↓
3. Discussion → 2-week minimum discussion period
   ↓
4. Final Comment Period → Last 1 week for objections
   ↓
5. Vote → Community votes if no consensus
   ↓
6. Decision → Accepted or Rejected
   ↓
7. Implementation → Build it!
   ↓
8. Completed → Mark RFC as implemented
```

### RFC Numbering

- RFCs numbered sequentially: RFC-0001, RFC-0002, etc.
- Number assigned when PR created
- Keep original number even if rejected

### RFC Status

- **Proposed** - Under discussion
- **Accepted** - Approved, ready to implement
- **Rejected** - Not moving forward
- **Implemented** - Code is merged
- **Withdrawn** - Author withdrew proposal
- **Postponed** - Good idea, wrong time

---

## Voting Process

### Who Can Vote?

**Community Votes (Major Decisions):**
- Anyone who has:
  - Made 3+ contributions in last 6 months, OR
  - Been a contributor for 12+ months, OR
  - Is on the core team

**Core Team Votes (Minor Decisions):**
- Only core team members

### How Voting Works

**Platforms:**
- GitHub Polls (for transparency)
- Discord polls (for engagement)
- Both count equally

**Duration:**
- Minimum 7 days for voting
- Extended to 14 days for complex issues

**Options:**
- ✅ **Yes** - Approve
- ❌ **No** - Reject
- 🤔 **Abstain** - No opinion

**Quorum:**
- Major decisions: 20% of eligible voters must participate
- If quorum not met, extended by 1 week

**Passing:**
- Simple majority (>50%) for most decisions
- 2/3 majority for breaking changes
- 3/4 majority to override founder veto

**Ties:**
- Founder breaks tie
- If founder abstains, proposal fails

---

## Conflict Resolution

### Technical Disagreements

1. **Discussion** - Try to reach consensus
2. **RFC** - Document both sides
3. **Community Input** - Get broader perspective
4. **Vote** - If still deadlocked
5. **Time-boxed Trial** - Try both approaches
6. **Decision** - Core team or community decides

### Interpersonal Conflicts

1. **Direct Resolution** - Talk it out privately
2. **Mediation** - Core team member mediates
3. **Code of Conduct** - If CoC violation
4. **Escalation** - Founder as final arbiter

### Process Disputes

1. **Clarification** - Is governance unclear?
2. **Amendment** - Update governance docs
3. **Community Discussion** - Get input
4. **Vote** - If needed

---

## Core Team Elections

### Schedule

- **June 1, 2026** - First election
- **December 1, 2026** - Second election
- Every 6 months thereafter

### Process

**Week 1-2: Nominations**
- Nomination period opens
- Anyone can nominate
- Self-nominations encouraged
- Post in GitHub Discussions

**Week 3: Candidate Statements**
- Nominees post statements
- Explain why they want to serve
- What they'll focus on
- Q&A in Discord

**Week 4: Voting**
- Community votes
- Top 5-7 vote-getters elected
- Results announced
- Transition period begins

**Week 5: Transition**
- Outgoing team briefs incoming team
- Access and permissions transferred
- First core team meeting

### Team Size

- **Minimum:** 3 members
- **Target:** 5 members
- **Maximum:** 7 members
- Odd number preferred (tie-breaking)

**Why 5-7?**
- Enough diversity of opinion
- Small enough to be effective
- Can still function if members are busy

### Removal from Core Team

**Voluntary:**
- Member can step down anytime
- 2-week notice preferred
- Special election if <3 members remain

**Involuntary:**
- Inactivity (no participation for 60 days)
- Code of Conduct violations
- Loss of community trust
- Requires 2/3 vote of other core team members + founder approval

---

## Amendment Process

This governance document can be amended:

### Minor Clarifications
- Core team consensus
- Announced in Discord
- No vote needed

### Significant Changes
- RFC required
- Community discussion
- Community vote
- Founder can veto

### Major Restructuring
- RFC required
- 4-week discussion minimum
- Community vote
- Requires 2/3 majority
- Founder must approve

---

## Transparency Requirements

### Public Information

**Always Public:**
- RFCs and decisions
- Voting results
- Core team meeting notes (summary)
- Roadmap and priorities
- Financial information (if any)

**Can Be Private:**
- Security vulnerabilities (until patched)
- Code of Conduct investigations
- Personal information
- Sensitive moderation decisions

### Meeting Notes

**Core Team Meetings:**
- Bi-weekly video calls
- Summary posted in Discord
- Decision rationale explained
- Action items public

**Community Calls:**
- Monthly, open to all
- Recorded and posted
- Agenda announced 1 week prior
- Q&A session included

---

## Future Governance Evolution

### When VSH Matures

As VSH grows, governance may evolve:

**Possible Changes:**
- Working groups for specific areas
- Technical Steering Committee
- Formal foundation (e.g., Linux Foundation)
- Paid maintainers
- Corporate sponsorship board

**Process:**
- Community proposes changes
- RFC and discussion
- Community vote
- Implement gradually

### Founder Transition

When Victor steps back:

**Option 1: Core Team Leadership**
- Core team becomes final authority
- No founder veto
- Major decisions still need community vote

**Option 2: Elected Lead Maintainer**
- Community elects lead maintainer
- 2-year term
- Same powers as founder
- Re-election possible

**Option 3: Foundation**
- Transfer to neutral foundation
- Board of directors
- Professional governance

---

## FAQ

### Can anyone contribute?
**Yes!** No permission needed to open issues, submit PRs, or join discussions.

### How do I get a say in decisions?
Contribute for 3 months or make 3 contributions, then you can vote!

### What if I disagree with a decision?
- Voice your concerns during discussion period
- Vote against it
- If it passes, accept the decision gracefully
- Propose changes later if needed

### Can decisions be reversed?
Yes! Through the RFC process. Show why circumstances changed.

### What if core team is inactive?
Community can call for emergency election. Founder facilitates.

### What if founder disappears?
Core team takes over. Can make founder role empty if needed.

### Who owns the copyright?
All contributors retain copyright on their work, licensed to the project under MIT.

### Who owns the VSH trademark?
Community-owned. No single entity controls it.

---

## Contact

**Questions about governance?**
- 💬 Discord: (Coming soon)
- 📧 Email: (Coming soon)
- 📝 GitHub: Open a discussion

**Report governance issues:**
- Email: governance@vsh.dev
- Private message to founder

---

**This is a living document. We'll improve it together.** 🌱

**Last Updated:** February 2026  
**Next Review:** June 2026  
**Version:** 1.0
