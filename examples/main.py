from algoforge import (
    Pipeline,
    Tokenizer,
    SpellingMapper,
    Lemmatizer,
    ToLowerCase,
)


def main():
    pipeline = Pipeline()
    pipeline.build_pipeline(
        [
            Tokenizer(),
            ToLowerCase(),
            SpellingMapper("data/spelling_map.csv"),
            Lemmatizer("data/lemma_map.csv"),
        ]
    )

    text = """
Performance in brief Contents page 1 At a glance 2 Performance dashboard 3 Chair’s introduction 4 CEO review 6 Purpose and strategy 8 Sustainability governance 9 ICMM Performance Expectations 12 Stakeholder engagement 13 About this report 16 Material topics Catastrophic Hazard Management 19 Workplace Health and Safety 24 Climate Change 30 Water 36 Land Stewardship 42 Human Rights 49 Responsible Citizenship 58 Responsible Sourcing and Supply 67 Our people 72 Additional information Commodity department overviews 78 ESG Data 84 Glossary 144 Assurance Statement 149 Important notice 152 Contact us 153 Performance in brief Material topics Additional Information Contact us Contents At a glance Where we operate Performance dashboard Chair’s introduction CEO review Purpose and sustainability Sustainability governance Stakeholder engagement About this report Building for the future 2020 has been a year of change for all of us. Among the disruption and the challenges, we have worked to build a strong foundation for the future, launching our new net-zero ambition to help us to further drive long-term value and make a positive contribution to society. Quick links Safety Our top priority is to protect the health and wellbeing of all of our people. Go to page 24 Net Zero We have a role to play in enabling the transition to a low-carbon economy. Go to page 30 Human Rights We uphold the dignity, fundamental freedoms and human rights of our people and communities. Go to page 49 Responsible Citizenship We aim to build partnerships to support sustainable development and growth. Go to page 58 Glencore Sustainability Report 2020 1
e in brief Material topics Additional Information Contact us Glencore plc Baarermattstrasse 3 CH-6340 Baar Switzerland Tel: +41 41 709 2000 Fax: +41 41 709 3000 E-mail: info@glencore.com glencore.com Designed and produced by MerchantCantos merchantcantos.com Contact us Our sustainability communications Our Sustainability Report 2020 forms part of Glencore’s annual corporate reporting suite. It expands on the information provided in our Annual Report 2020 and details how we address our material sustainability risks and opportunities. In addition to this report, we also publish an annual Modern Slavery Statement and Payments to Governments report, as well as regular updates on our activities via our website and social media platforms. Further information on our stakeholder engagement activities is available in our 2020 Annual Report (Section 172 Statement on page 24) and on our website: www.glencore.com/sustainability/ stakeholder-engagement Find us on: Glencore Sustainability Report 2020 153 
"""
    result = pipeline.process(text)
    print(result)


if __name__ == "__main__":
    main()
