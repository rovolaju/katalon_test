<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_function submit_form(hasre</name>
   <tag></tag>
   <elementGuidId>ec12226d-977f-4fb9-b5ad-5551ed2eac1c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>otPopupDialogContents</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>otPopupDialogContents no_icon</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	

		function submit_form(hasreasons) {
			// what we need to do here is... make multiple copies of the violation codes, one per rule, so later code can match them up
			var reasonArray = $$(&quot;select[id='ddviolationreason']&quot;);
			var justification = $(&quot;brokenrulejustification&quot;);
			var safeJustification = otHtmlEncode(otTrim(justification.value));
			var validated = true;
			var strReasons = '', rulesCovered='';
			reasonArray.each(function(s,index){
				rulesCovered += s.getAttribute('rulesCovered');
				strReasons += s.options[s.selectedIndex].value;
				if((index + 1) &lt; reasonArray.length) {
					rulesCovered += '|';
					strReasons += '|';
				}
				if (hasreasons==true &amp;&amp; s.options[s.selectedIndex].value == &quot;&quot;){
					validated = false;
					alert(&quot;Please choose a reason for booking a trip that is not in compliance with company travel policy.&quot;);
					tryFocus(s);
					throw $break;
				}
			})
			// ok, now for we need to put the reasons covered and the reasons in the same order.  right now reasons might be &quot;PPG,99F&quot; and reasonscovered might be &quot;1,2|3,4&quot;
			// strReasons = abc|def
			// rulesCovered = 1,3|2
			// this would mean for rules 1 and 3 sequence-wise, they chose abc... for 2 they chose def
			// let's make a NEW strReasons that lists one code per rule in order of rules
			var arrReasons = strReasons.split('|');
			var arrRules = rulesCovered.split('|');
			strReasons = '';
			var strRules, matchArr = new Array;
			// the reasons need to appear in order of index of rule
			for(var i=0; i&lt;arrRules.length; i++) {
				var idx, idxArr;
				idxArr = arrRules[i].split(',');
				for(j=0; j&lt;idxArr.length;j++) {
					matchArr[idxArr[j]] = arrReasons[i];
				}
			}
			if (hasreasons==true &amp;&amp; justification.getAttribute(&quot;required&quot;) == &quot;true&quot; &amp;&amp; safeJustification == &quot;&quot;) {
				alert(&quot;Please enter an explanation for booking a trip that is not in compliance with company travel policy.&quot;);
				tryFocus(justification);
				return false;
			}
			strReasons='';
			for(var i=0; i&lt;matchArr.length; i++) {
				if (matchArr[i]) {
					if(strReasons != '') strReasons += ',';
					strReasons += matchArr[i];
				}
			}

			// ok we're going to update the parent page with the rule violation reason - but they have to be in order of rules violated.
			if(validated) {
				try {
					llfByLeg = &quot;llfByLeg: False&quot;;
					otLogMetric(&quot;Cliqbook&quot;, &quot;ViolationReasons&quot;, llfByLeg + &quot; | hasreasons: &quot; + hasreasons + &quot; | reasons: &quot; + strReasons + &quot; | explanation: &quot; + safeJustification);
				} catch(err) {}

				if ('' == 'true' &amp;&amp; dynList2.rc2) {
					dynList2.rc2('', '1');
					update_rule_justification_byleg(safeJustification, strReasons, '');
				} else {
					update_rule_justification(safeJustification, strReasons);
				}
			}
		}

		function cancelResult()	{
			if ('' == 'true' &amp;&amp; dynList2.rc2) {
				dynList2.cancelSelection('', '1');
			} else {
				if (dynList.selectedResult) {
					dynList.selectedResult = null;
					arrPopupRuleViolations = [];
					popupRulesRun = false;
					popupLLFRulesRun = false;
					canSubmit = false;
					AAirpassRun = false;
					popupModifyBookingsRun = false;
				}
			}
		}
	

	

	
		
			
			This flight is not in compliance with the following travel rule(s):
		
		
		
								 Option uses mixed carriers 
								
								
								Please choose the reason for selecting this travel option. If more than one reason applies, choose the most applicable.  This reason applies to this entire trip.
								
								
									-- Please Choose a Reason --
									
										I have a ticket that I am Exchanging	  
										
										Lowest airfare declined - time preference
										
										Lowest airfare declined due to airport preference
										
										Lowest airfare declined due to airline preference
										
										Lowest airfare declined business class requested
										
										Lowest airfare declined - traveling with clients
										
										Lowest airfare declined - flexible ticket required
										
										Lowest airfare declined - routing preference
										
								
								
							

		

		
		   
		   Please explain why you have chosen this flight. NOTE: We will log flights which you did not take.
			
			
			
		

		
			
			
			
		

		
			
			
				
			Chosen:Cost: 32857.00Outbound Flight4438Aguascalientes Airport						(AGU)					08/08/2018 12:29 PMHouston George Bush Intercontinental Airport						(IAH)					08/08/2018 2:40 PMEmbraer RJ1352094Houston George Bush Intercontinental Airport						(IAH)					08/08/2018 4:27 PMNew York LaGuardia Airport						(LGA)					08/08/2018 8:59 PMBoeing 737-800Return Flight:5246New York John F. Kennedy Intl Airport						(JFK)					10/10/2018 7:30 PMBoston Logan Intl Airport						(BOS)					10/10/2018 9:11 PMBoeing 717699Boston Logan Intl Airport						(BOS)					10/10/2018 11:30 PMMexico City Benito Juarez Intl Airport						(MEX)					11/10/2018 4:35 AM7S82632Mexico City Benito Juarez Intl Airport						(MEX)					11/10/2018 6:05 AMAguascalientes Airport						(AGU)					11/10/2018 7:13 AMEmbraer 190Least cost logical fareCost: 32857.00Outbound Flight4438Aguascalientes Airport						(AGU)					08/08/2018 12:29 PMHouston George Bush Intercontinental Airport						(IAH)					08/08/2018 2:40 PMEmbraer RJ1352094Houston George Bush Intercontinental Airport						(IAH)					08/08/2018 4:27 PMNew York LaGuardia Airport						(LGA)					08/08/2018 8:59 PMBoeing 737-800Return Flight:5246New York John F. Kennedy Intl Airport						(JFK)					10/10/2018 7:30 PMBoston Logan Intl Airport						(BOS)					10/10/2018 9:11 PMBoeing 717699Boston Logan Intl Airport						(BOS)					10/10/2018 11:30 PMMexico City Benito Juarez Intl Airport						(MEX)					11/10/2018 4:35 AM7S82632Mexico City Benito Juarez Intl Airport						(MEX)					11/10/2018 6:05 AMAguascalientes Airport						(AGU)					11/10/2018 7:13 AMEmbraer 190The best option was selected, but it does not comply with your travel rules.
				
			
		
	
	</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;otPopupDialogContents&quot;)</value>
   </webElementProperties>
</WebElementEntity>
