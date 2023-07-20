#Code

# Load appropriate packages
library(tidyverse)

# Read in data
tools <- read.csv("tools.csv")

# Check structure of the data
str(tools)

# Summarize
summary(tools)

# Transpose the data
tools_t <- t(tools)

# Create a vector of tools
tools_vec <- tools_t[1]

# Initialize a data frame 
df <- data.frame(matrix(ncol = 2, nrow = 0))

# Create columns for the data frame
colnames(df) <- c("Tool", "Score")

# Loop through the tools vector
for(i in 1:length(tools_vec)){
  
  # Extract the corresponding skill level for each tool
  skill_level <- as.numeric(tools_t[i+1,2])
  
  # Calculate the score of the tool
  score <- ifelse(skill_level == 2, 3, 
                  ifelse(skill_level == 3, 4,
                         ifelse(skill_level == 4, 5, 0)))
  
  # Append the data to the data frame
  df <- rbind(df, c(tools_vec[i], score))
}

# Order the data frame
df <- df[order(-df$Score), ]

# View the data frame
df

# Create a visualization of the data frame
ggplot(df, aes(x = Tool, y = Score)) +
  geom_bar(stat = "identity") +
  labs(x = "Tool", y = "Score") +
  ggtitle("Tools for Success")