# Creating a new scenario

This directory serves as a template for creating new LabNet scenarios.

# Required Structure

The scenario directory must contain a file named exactly `docker-compose.yml`.
LabNet executes `docker compose -f labs/<lab-name>/docker-compose.yml up -d`. If this file is missing or named differently, starting the lab will fail.

# Optional Structure

To improve the user experience,the following files can be included in the scenario directory:

- `MISSION.md`: A brief description of the lab's objective and initial steps. This gets printed to the terminal when the lab starts.

- `HINTS.md`: Progressive hints for users who are stuck.

- `SOLUTION.md`: The complete walkthrough of the lab.

- `README.md`: Additional notes for contributors or users.