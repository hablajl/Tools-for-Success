<?php 
// Line 1
echo 'Welcome to Tools for Success!';

// Line 2 
$toolsForSuccess = array("goal setting", "time management", "positive attitude", "self-awareness", "boundary setting", "effective communication", "decision making");

// Line 3
function displayTools($toolsForSuccess) {
	echo "<h3>Tools for Success</h3>";
	echo "<ul>";
	foreach($toolsForSuccess as $tool) {
		echo "<li>$tool</li>";
	}
	echo "</ul>";
}

// Line 4
displayTools($toolsForSuccess);

// Line 5
function createTips($tool) {
	$tips = array();

	if($tool == "goal setting") {
		$tips = array("Set realistic, achievable goals.", "Break down goals into smaller, more manageable tasks.");
	}
	elseif($tool == "time management") {
		$tips = array("Prioritize tasks.", "Set reasonable deadline dates.", "Create a daily to-do list.");
	}
	elseif($tool == "positive attitude") {
		$tips = array("Find the positive in every situation.", "Look for solutions, not problems.", "Surround yourself with positive people.");
	}
	elseif($tool == "self-awareness") {
		$tips = array("Be honest with yourself.", "Monitor your thoughts and reactions.", "Know your limits.");
	}
	elseif($tool == "boundary setting") {
		$tips = array("Be clear about what you will and won't do.", "Don't be afraid to say no.", "Stay consistent with your limits.");
	}
	elseif($tool == "effective communication") {
		$tips = array("Listen to understand, not to respond.", "Be assertive, not aggressive.", "Stick to the facts.");
	}
	elseif($tool == "decision making") {
		$tips = array("Gather and analyze all the facts before making a decision.", "Weigh the pros and cons before making a decision.", "Consider who will be affected by your decision.");
	}

	return $tips;
}

// Line 6
function displayTips($tool) {
	$tips = createTips($tool);
	echo "<h3>Tips for $tool</h3>";
	echo "<ul>";
	foreach($tips as $tip) {
		echo "<li>$tip</li>";
	}
	echo "</ul>";
}

// Line 7
foreach($toolsForSuccess as $tool) {
	displayTips($tool);
}

// Line 8
function displayResources($tool) {
	echo "<h3>Resources for $tool</h3>";
	echo "<ul>";
	if($tool == "goal setting") {
		echo "<li><a href='http://www.goal-setting-guide.com/'>Goal Setting Guide</a></li>";
		echo "<li><a href='http://www.projectsmart.co.uk/goal-setting-for-success.php'>Project Smart: Goal Setting for Success</a></li>";
	}
	elseif($tool == "time management") {
		echo "<li><a href='https://www.mindtools.com/pages/article/newHTE_94.htm'>Mind Tools: Time Management</a></li>";
		echo "<li><a href='http://www.poweredbyintuition.com/time-management-tips/'>Time Management Tips</a></li>";
	}
	elseif($tool == "positive attitude") {
		echo "<li><a href='https://greatergood.berkeley.edu/article/item/five_acts_appreciate'>Five Acts of Appreciation</a></li>";
		echo "<li><a href='http://positivepsychologyprogram.com/positive-affirmations/'>Positive Affirmations</a></li>";
	}
	elseif($tool == "self-awareness") {
		echo "<li><a href='https://www.psychologytoday.com/blog/happiness-is-state-mind/201108/nine-ways-increase-your-self-awareness'>Nine Ways to Increase Your Self-Awareness</a></li>";
		echo "<li><a href='http://www.lifehack.org/articles/communication/9-ways-build-self-awareness.html'>9 Ways to Build Self-Awareness</a></li>";
	}
	elseif($tool == "boundary setting") {
		echo "<li><a href='https://www.mindtools.com/pages/article/newTCS_80.htm'>Mind Tools: Setting Boundaries</a></li>";
		echo "<li><a href='https://www.helpguide.org/articles/relationships-communication/setting-healthy-boundaries.htm'>Setting Healthy Boundaries</a></li>";
	}
	elseif($tool == "effective communication") {
		echo "<li><a href='https://www.mindtools.com/CommSkll/EffectiveCommunication.htm'>Mind Tools: Effective Communication</a></li>";
		echo "<li><a href='https://www.forbes.com/sites/ekaterinawalter/2015/03/04/7-keys-to-effective-communication-in-the-workplace/'>7 Keys to Effective Communication in the Workplace</a></li>";
	}
	elseif($tool == "decision making") {
		echo "<li><a href='https://www.mindtools.com/pages/article/newTED_85.htm'>Mind Tools: Decision Making</a></li>";
		echo "<li><a href='http://www.12manage.com/methods_decision_making_process.html'>The Decision Making Process</a></li>";
	}
	echo "</ul>";
}

// Line 9
foreach($toolsForSuccess as $tool) {
	displayResources($tool);
}
?>