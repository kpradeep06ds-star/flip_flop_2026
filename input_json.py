import json
from pathlib import Path
from typing import Any


def format_rupees(value: Any) -> str:
    """Format a numeric value as Indian rupees."""
    if value is None:
        return "₹0"
    return f"₹{float(value):,.0f}"


def mask_identifier(value: Any, visible_digits: int = 4) -> str:
    """Mask a sensitive identifier while retaining its final digits."""
    if value is None:
        return ""

    text = str(value)

    if len(text) <= visible_digits:
        return "*" * len(text)

    return "*" * (len(text) - visible_digits) + text[-visible_digits:]


def load_itr(file_path: str) -> dict:
    """Load an ITR JSON file."""
    path = Path(file_path)

    if not path.exists():
        raise FileNotFoundError(f"File not found: {path.resolve()}")

    with path.open("r", encoding="utf-8") as file:
        return json.load(file)


def create_itr_summary(data: dict) -> dict:
    """Extract important information from an ITR-1 JSON structure."""
    try:
        itr = data["ITR"]["ITR1"]
    except KeyError as error:
        raise ValueError(
            "The JSON does not contain the expected ITR -> ITR1 structure."
        ) from error

    form = itr.get("Form_ITR1", {})
    filing = itr.get("FilingStatus", {})
    income = itr.get("ITR1_IncomeDeductions", {})
    tax = itr.get("ITR1_TaxComputation", {})
    tax_paid = itr.get("TaxPaid", {}).get("TaxesPaid", {})
    refund = itr.get("Refund", {})
    personal = itr.get("PersonalInfo", {})

    assessee_name = personal.get("AssesseeName", {})
    full_name = " ".join(
        value
        for value in [
            assessee_name.get("FirstName"),
            assessee_name.get("MiddleName"),
            assessee_name.get("SurNameOrOrgName"),
        ]
        if value
    )

    # Salary TDS details
    salary_tds_records = (
        itr.get("TDSonSalaries", {}).get("TDSonSalary", [])
    )

    salary_sources = []
    for record in salary_tds_records:
        deductor = record.get(
            "EmployerOrDeductorOrCollectDetl", {}
        )

        salary_sources.append(
            {
                "employer": deductor.get(
                    "EmployerOrDeductorOrCollecterName"
                ),
                "salary": format_rupees(record.get("IncChrgSal", 0)),
                "tds": format_rupees(record.get("TotalTDSSal", 0)),
            }
        )

    # Other income details
    income_description_map = {
        "SAV": "Savings-account interest",
        "IFD": "Fixed-deposit interest",
    }

    other_income_records = (
        income.get("OthersInc", {})
        .get("OthersIncDtlsOthSrc", [])
    )

    other_income_sources = []
    for record in other_income_records:
        code = record.get("OthSrcNatureDesc")

        other_income_sources.append(
            {
                "source": income_description_map.get(code, code),
                "amount": format_rupees(
                    record.get("OthSrcOthAmount", 0)
                ),
            }
        )

    accepted_deductions = income.get("DeductUndChapVIA", {})
    user_deductions = income.get("UsrDeductUndChapVIA", {})
    interest = tax.get("IntrstPay", {})

    regime = (
        "New tax regime"
        if filing.get("OptOutNewTaxRegime") == "N"
        else "Old tax regime"
    )

    summary = {
        "taxpayer": {
            "name": full_name,
            "pan_masked": mask_identifier(personal.get("PAN")),
            "aadhaar_masked": mask_identifier(
                personal.get("AadhaarCardNo")
            ),
        },
        "filing": {
            "form": form.get("FormName"),
            "assessment_year": "2026-27",
            "description": form.get("Description"),
            "filing_due_date": filing.get("ItrFilingDueDate"),
            "tax_regime": regime,
        },
        "income": {
            "gross_salary": format_rupees(
                income.get("GrossSalary", 0)
            ),
            "standard_deduction": format_rupees(
                income.get("DeductionUs16", 0)
            ),
            "income_from_salary": format_rupees(
                income.get("IncomeFromSal", 0)
            ),
            "other_source_income": format_rupees(
                income.get("IncomeOthSrc", 0)
            ),
            "gross_total_income": format_rupees(
                income.get("GrossTotIncome", 0)
            ),
            "taxable_income": format_rupees(
                income.get("TotalIncome", 0)
            ),
            "salary_sources": salary_sources,
            "other_income_sources": other_income_sources,
        },
        "deductions": {
            "employer_nps_80ccd2": format_rupees(
                accepted_deductions.get(
                    "Section80CCDEmployer", 0
                )
            ),
            "80tta_entered_by_user": format_rupees(
                user_deductions.get("Section80TTA", 0)
            ),
            "total_entered_by_user": format_rupees(
                user_deductions.get(
                    "TotalChapVIADeductions", 0
                )
            ),
            "total_actually_allowed": format_rupees(
                accepted_deductions.get(
                    "TotalChapVIADeductions", 0
                )
            ),
        },
        "tax_computation": {
            "income_tax_before_cess": format_rupees(
                tax.get("TaxPayableOnRebate", 0)
            ),
            "education_cess": format_rupees(
                tax.get("EducationCess", 0)
            ),
            "tax_after_cess": format_rupees(
                tax.get("NetTaxLiability", 0)
            ),
            "interest_234a": format_rupees(
                interest.get("IntrstPayUs234A", 0)
            ),
            "interest_234b": format_rupees(
                interest.get("IntrstPayUs234B", 0)
            ),
            "interest_234c": format_rupees(
                interest.get("IntrstPayUs234C", 0)
            ),
            "late_filing_fee": format_rupees(
                interest.get("LateFilingFee234F", 0)
            ),
            "total_tax_and_interest": format_rupees(
                tax.get("TotTaxPlusIntrstPay", 0)
            ),
        },
        "taxes_paid": {
            "salary_and_other_tds": format_rupees(
                tax_paid.get("TDS", 0)
            ),
            "advance_tax": format_rupees(
                tax_paid.get("AdvanceTax", 0)
            ),
            "self_assessment_tax": format_rupees(
                tax_paid.get("SelfAssessmentTax", 0)
            ),
            "total_taxes_paid": format_rupees(
                tax_paid.get("TotalTaxesPaid", 0)
            ),
            "balance_payable": format_rupees(
                itr.get("TaxPaid", {}).get("BalTaxPayable", 0)
            ),
            "refund_due": format_rupees(
                refund.get("RefundDue", 0)
            ),
        },
    }

    return summary


if __name__ == "__main__":
    itr_data = load_itr("itr.json")
    summary = create_itr_summary(itr_data)

    # Display the structured summary
    print(json.dumps(summary, indent=4, ensure_ascii=False))

    # Save it as a separate, masked summary JSON
    output_path = Path("itr_summary_masked.json")
    output_path.write_text(
        json.dumps(summary, indent=4, ensure_ascii=False),
        encoding="utf-8",
    )

    print(f"\nSummary saved to: {output_path.resolve()}")