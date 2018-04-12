import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.concursolutions.com/')

WebUI.setText(findTestObject('Concur_search/Page_Sign in to Concur  Concur Solu/input_userid'), 'rodrigo_davila-cedillo@capgemini.com')

WebUI.setText(findTestObject('Concur_search/Page_Sign in to Concur  Concur Solu/input_password'), 'Capgemini91')

WebUI.click(findTestObject('Concur_search/Page_Sign in to Concur  Concur Solu/input_btnSubmit'))

WebUI.click(findTestObject('Concur_search/Page_Home/input_fltDepCityDisplay0'), FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Concur_search/Page_Home/input_fltDepCityDisplay0'), 'Aguascalientes')

WebUI.click(findTestObject('Concur_search/Page_Home/a_AGU Aguascalientes AirportAg'))

WebUI.click(findTestObject('Concur_search/Page_Home/input_fltArrCityDisplay0'), FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Concur_search/Page_Home/input_fltArrCityDisplay0'), 'new york')

WebUI.click(findTestObject('Concur_search/Page_Home/a_NYC New York Area AirportsNe'))

WebUI.setText(findTestObject('Concur_search/Page_Home/input_fltDate0'), '08/08/2018')

WebUI.selectOptionByValue(findTestObject('Concur_search/Page_Home/select_MorningAfternoonEvening'), ' 9', true)

WebUI.doubleClick(findTestObject('Concur_search/Page_Home/input_fltDate1'))

WebUI.setText(findTestObject('Concur_search/Page_Home/input_fltDate1'), '10/10/2018')

WebUI.selectOptionByValue(findTestObject('Concur_search/Page_Home/select_MorningAfternoonEvening_1'), ' 21', true)

WebUI.click(findTestObject('Concur_search/Page_Home/button_Search'))

WebUI.click(findTestObject('Concur_search/Page_Concur Travel - Air Search Res/button_MXN32857.00'))

WebUI.click(findTestObject('Concur_search/Page_Concur Travel - Air Search Res/div_function submit_form(hasre'))

WebUI.setText(findTestObject('Concur_search/Page_Concur Travel - Air Search Res/textarea_brokenrulejustificati'), 'this is just a test :)')

WebUI.selectOptionByValue(findTestObject('Concur_search/Page_Concur Travel - Air Search Res/select_-- Please Choose a Reas'), 
    '973494', true)

WebUI.click(findTestObject('Concur_search/Page_Concur Travel - Air Search Res/input_btnSave'))

WebUI.closeBrowser()

