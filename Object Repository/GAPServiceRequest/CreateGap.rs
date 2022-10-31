<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create Gap</description>
   <name>CreateGap</name>
   <tag></tag>
   <elementGuidId>d5d4d78d-43a8-48b0-9e9c-4f9c27a5516a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;gap_name\&quot;: \&quot;${GapName}\&quot;,\n    \&quot;gap_weight\&quot;: ${GapWeight},\n    \&quot;gap_score\&quot;: ${GapScore},\n    \&quot;picture_text\&quot;: \&quot;${GapDescription}\&quot;,\n    \&quot;sections\&quot;: [\n        {\n            \&quot;section_name\&quot;: \&quot;Q 2.8.1. LOOK: At the colour of the coffee tree leaves. Are the leaves dark green or are some leaves yellow, pale green, or brown?\&quot;,\n            \&quot;questions\&quot;: [\n                {\n                    \&quot;question\&quot;: \&quot;Nearly all leaves are dark green and less than 5% (less than 5 in 100) are yellow, pale green, or brown.\&quot;,\n                    \&quot;description\&quot;: \&quot;de\&quot;,\n                    \&quot;question_type\&quot;: \&quot;yes_no\&quot;,\n                    \&quot;weight\&quot;: 10,\n                    \&quot;answers\&quot;: [],\n                    \&quot;is_not_applicable\&quot;: false\n                }\n            ]\n        },\n        {\n            \&quot;section_name\&quot;: \&quot; LOOK: Which pruning method(s) have been used on the majority of trees?\&quot;,\n            \&quot;questions\&quot;: [\n                {\n                    \&quot;question\&quot;: \&quot;Quality of tools\u0026materials used in pruning \&quot;,\n                    \&quot;description\&quot;: \&quot;de\&quot;,\n                    \&quot;question_type\&quot;: \&quot;multiple_single\&quot;,\n                    \&quot;weight\&quot;: 20,\n                    \&quot;answers\&quot;: [\n                        {\n                            \&quot;answer\&quot;: \&quot;If Conventional tools: Pruning shear/secateur\u0026Saw\&quot;,\n                            \&quot;description\&quot;: \&quot;de\&quot;,\n                            \&quot;weight\&quot;: 15,\n                            \&quot;is_not_applicable\&quot;: false\n                        },\n                        {\n                            \&quot;answer\&quot;: \&quot; If Non conventional tools \&quot;,\n                            \&quot;description\&quot;: \&quot;de\&quot;,\n                            \&quot;weight\&quot;: 5,\n                            \&quot;is_not_applicable\&quot;: false\n                        },\n                        {\n                            \&quot;answer\&quot;: \&quot;Cleaning of pruning tools to avoid the spread of diseases between trees\&quot;,\n                            \&quot;description\&quot;: \&quot;de\&quot;,\n                            \&quot;weight\&quot;: 5,\n                            \&quot;is_not_applicable\&quot;: false\n                        }\n                    ],\n                    \&quot;is_not_applicable\&quot;: false\n                }\n            ]\n        },\n        {\n            \&quot;section_name\&quot;: \&quot;This is a section name\&quot;,\n            \&quot;questions\&quot;: [\n                {\n                    \&quot;question\&quot;: \&quot;This is the question to be asked\&quot;,\n                    \&quot;description\&quot;: \&quot;Marks Input\&quot;,\n                    \&quot;question_type\&quot;: \&quot;mark_input\&quot;,\n                    \&quot;weight\&quot;: 20,\n                    \&quot;answers\&quot;: [],\n                    \&quot;is_not_applicable\&quot;: false\n                }\n            ]\n        },\n        {\n            \&quot;section_name\&quot;: \&quot;This is a section name\&quot;,\n            \&quot;questions\&quot;: [\n                {\n                    \&quot;question\&quot;: \&quot;This is the question to be asked\&quot;,\n                    \&quot;description\&quot;: \&quot;Marks Input\&quot;,\n                    \&quot;question_type\&quot;: \&quot;multiple_apply\&quot;,\n                    \&quot;weight\&quot;: 20,\n                    \&quot;answers\&quot;: [\n                        {\n                            \&quot;answer\&quot;: \&quot;This is an answer\&quot;,\n                            \&quot;description\&quot;: \&quot;Answer Description\&quot;,\n                            \&quot;weight\&quot;: 5,\n                            \&quot;is_not_applicable\&quot;: false\n                        },\n                        {\n                            \&quot;answer\&quot;: \&quot;This is an answer\&quot;,\n                            \&quot;description\&quot;: \&quot;Answer Description\&quot;,\n                            \&quot;weight\&quot;: 5,\n                            \&quot;is_not_applicable\&quot;: false\n                        },\n                        {\n                            \&quot;answer\&quot;: \&quot;This is an answer\&quot;,\n                            \&quot;description\&quot;: \&quot;Answer Description\&quot;,\n                            \&quot;weight\&quot;: 5,\n                            \&quot;is_not_applicable\&quot;: false\n                        },\n                        {\n                            \&quot;answer\&quot;: \&quot;This is an answer\&quot;,\n                            \&quot;description\&quot;: \&quot;Answer Description\&quot;,\n                            \&quot;weight\&quot;: 0,\n                            \&quot;is_not_applicable\&quot;: false\n                        }\n                    ],\n                    \&quot;is_not_applicable\&quot;: false\n                }\n            ]\n        }\n    ]\n}&quot;,
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
      <webElementGuid>30968b3a-c8a3-42f6-9ab3-7b974fcaa74c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-auth-token</name>
      <type>Main</type>
      <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYzNTdkN2Y1NTc0ZDQyMDA2ODU3NDUyNSIsImlhdCI6MTY2NzEzMjExMCwiZXhwIjoxNjY3MTc1MzEwfQ.o6h43PBLEokUgmxxeLjsEhfqUMrhIYRL0YQgjqpNeZo</value>
      <webElementGuid>610539cf-c626-4fb8-a8e0-a46bd804dcbd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://staging.smartkungahara.rw/core/api/v1.1/gaps/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'T'</defaultValue>
      <description></description>
      <id>14804c26-8895-4f50-9249-c9251fdf95c8</id>
      <masked>false</masked>
      <name>GapName</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>3739aa3d-c701-4ad2-b6d6-73bb154952af</id>
      <masked>false</masked>
      <name>GapWeight</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>82d16fdb-07db-4043-93fd-8148fa365191</id>
      <masked>false</masked>
      <name>GapScore</name>
   </variables>
   <variables>
      <defaultValue>'D'</defaultValue>
      <description></description>
      <id>152ae961-5ed5-4b97-ad13-de4cb4b4b7f9</id>
      <masked>false</masked>
      <name>GapDescription</name>
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
