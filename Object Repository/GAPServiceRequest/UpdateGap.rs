<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateGap</name>
   <tag></tag>
   <elementGuidId>c5039620-684a-47b1-b03a-57fbbd51273f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;_id\&quot;: \&quot;${ID}\&quot;,\n  \&quot;gap_name\&quot;: \&quot;${Gap_Name} \&quot;,\n  \&quot;sections\&quot;: [\n    {\n      \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f19889fb\&quot;,\n      \&quot;section_name\&quot;: \&quot;Q 2.8.1. LOOK: At the colour of the coffee tree leaves. Are the leaves dark green or are some leaves yellow, pale green, or brown?\&quot;,\n      \&quot;questions\&quot;: [\n        {\n          \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f19889fc\&quot;,\n          \&quot;question\&quot;: \&quot;Nearly all leaves are dark green and less than 5% (less than 5 in 100) are yellow, pale green, or brown.\&quot;,\n          \&quot;description\&quot;: \&quot;de\&quot;,\n          \&quot;question_type\&quot;: \&quot;yes_no\&quot;,\n          \&quot;weight\&quot;: 10,\n          \&quot;answers\&quot;: [],\n          \&quot;is_not_applicable\&quot;: false\n        }\n      ]\n    },\n    {\n      \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f19889fd\&quot;,\n      \&quot;section_name\&quot;: \&quot; LOOK: Which pruning method(s) have been used on the majority of trees?\&quot;,\n      \&quot;questions\&quot;: [\n        {\n          \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f19889fe\&quot;,\n          \&quot;question\&quot;: \&quot;Quality of tools\u0026materials used in pruning \&quot;,\n          \&quot;description\&quot;: \&quot;de\&quot;,\n          \&quot;question_type\&quot;: \&quot;multiple_single\&quot;,\n          \&quot;weight\&quot;: 20,\n          \&quot;answers\&quot;: [\n            {\n              \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f19889ff\&quot;,\n              \&quot;answer\&quot;: \&quot;If Conventional tools: Pruning shear/secateur\u0026Saw\&quot;,\n              \&quot;description\&quot;: \&quot;de\&quot;,\n              \&quot;weight\&quot;: 15,\n              \&quot;is_not_applicable\&quot;: false\n            },\n            {\n              \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a00\&quot;,\n              \&quot;answer\&quot;: \&quot; If Non conventional tools \&quot;,\n              \&quot;description\&quot;: \&quot;de\&quot;,\n              \&quot;weight\&quot;: 5,\n              \&quot;is_not_applicable\&quot;: false\n            },\n            {\n              \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a01\&quot;,\n              \&quot;answer\&quot;: \&quot;Cleaning of pruning tools to avoid the spread of diseases between trees\&quot;,\n              \&quot;description\&quot;: \&quot;de\&quot;,\n              \&quot;weight\&quot;: 5,\n              \&quot;is_not_applicable\&quot;: false\n            }\n          ],\n          \&quot;is_not_applicable\&quot;: false\n        }\n      ]\n    },\n    {\n      \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a02\&quot;,\n      \&quot;section_name\&quot;: \&quot;This is a section name\&quot;,\n      \&quot;questions\&quot;: [\n        {\n          \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a03\&quot;,\n          \&quot;question\&quot;: \&quot;This is the question to be asked\&quot;,\n          \&quot;description\&quot;: \&quot;Marks Input\&quot;,\n          \&quot;question_type\&quot;: \&quot;mark_input\&quot;,\n          \&quot;weight\&quot;: 20,\n          \&quot;answers\&quot;: [],\n          \&quot;is_not_applicable\&quot;: false\n        }\n      ]\n    },\n    {\n      \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a04\&quot;,\n      \&quot;section_name\&quot;: \&quot;This is a section name\&quot;,\n      \&quot;questions\&quot;: [\n        {\n          \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a05\&quot;,\n          \&quot;question\&quot;: \&quot;This is the question to be asked\&quot;,\n          \&quot;description\&quot;: \&quot;Marks Input\&quot;,\n          \&quot;question_type\&quot;: \&quot;multiple_apply\&quot;,\n          \&quot;weight\&quot;: 20,\n          \&quot;answers\&quot;: [\n            {\n              \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a06\&quot;,\n              \&quot;answer\&quot;: \&quot;This is an answer\&quot;,\n              \&quot;description\&quot;: \&quot;Answer Description\&quot;,\n              \&quot;weight\&quot;: 5,\n              \&quot;is_not_applicable\&quot;: false\n            },\n            {\n              \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a07\&quot;,\n              \&quot;answer\&quot;: \&quot;This is an answer\&quot;,\n              \&quot;description\&quot;: \&quot;Answer Description\&quot;,\n              \&quot;weight\&quot;: 5,\n              \&quot;is_not_applicable\&quot;: false\n            },\n            {\n              \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a08\&quot;,\n              \&quot;answer\&quot;: \&quot;This is an answer\&quot;,\n              \&quot;description\&quot;: \&quot;Answer Description\&quot;,\n              \&quot;weight\&quot;: 5,\n              \&quot;is_not_applicable\&quot;: false\n            },\n            {\n              \&quot;_id\&quot;: \&quot;635f58f5dfb31d31f1988a09\&quot;,\n              \&quot;answer\&quot;: \&quot;This is an answer\&quot;,\n              \&quot;description\&quot;: \&quot;Answer Description\&quot;,\n              \&quot;weight\&quot;: 0,\n              \&quot;is_not_applicable\&quot;: false\n            }\n          ],\n          \&quot;is_not_applicable\&quot;: false\n        }\n      ]\n    }\n  ],\n  \&quot;gap_weight\&quot;: 70,\n  \&quot;gap_score\&quot;: 21,\n  \&quot;picture_text\&quot;: \&quot;QA TEST 01\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d0965c4f-d189-4665-ad79-10a00ce84177</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-auth-token</name>
      <type>Main</type>
      <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYzNTdkN2Y1NTc0ZDQyMDA2ODU3NDUyNSIsImlhdCI6MTY2NzE5MzIzOCwiZXhwIjoxNjY3MjM2NDM4fQ.IOdYuHFv7K7MFC24xRSYRFKl1qsA_2qZz0SrKc6Qec8</value>
      <webElementGuid>a34ccf26-ca68-420f-80b1-f4ea26b4806b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://staging.smartkungahara.rw/core/api/v1.1/gaps/${ID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'635f58f5dfb31d31f19889fa'</defaultValue>
      <description></description>
      <id>949df4cb-a7c3-4665-8008-8dbd66bbbcf3</id>
      <masked>false</masked>
      <name>ID</name>
   </variables>
   <variables>
      <defaultValue>'TUGIRIMANA JD GAP UPDATE'</defaultValue>
      <description></description>
      <id>29ea06e6-fbe9-4d38-99f2-11c23cfc9cef</id>
      <masked>false</masked>
      <name>Gap_Name</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
